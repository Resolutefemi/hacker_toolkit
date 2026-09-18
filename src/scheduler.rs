//! Scan scheduler — persistent scheduled scans.
//! Entries are stored in ~/.htool/schedules.json and support interval-based
//! ("every N minutes") and daily ("at HH:MM") triggers. The engine exposes
//! pure helpers; orchestration (tokio tasks) lives in the CLI / GUI.

use chrono::{DateTime, Local, NaiveTime};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Trigger kind for a scheduled scan
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum ScheduleKind {
    /// Run every `every_secs` seconds
    Interval { every_secs: u64 },
    /// Run every day at hour:minute (local time)
    Daily { hour: u8, minute: u8 },
}

impl ScheduleKind {
    /// Human-readable description, e.g. "every 30 min" or "daily at 03:00"
    pub fn describe(&self) -> String {
        match self {
            ScheduleKind::Interval { every_secs } => {
                let s = *every_secs;
                if s >= 86400 && s % 86400 == 0 {
                    format!("every {} day(s)", s / 86400)
                } else if s >= 3600 && s % 3600 == 0 {
                    format!("every {} h", s / 3600)
                } else if s >= 60 && s % 60 == 0 {
                    format!("every {} min", s / 60)
                } else {
                    format!("every {} s", s)
                }
            }
            ScheduleKind::Daily { hour, minute } => format!("daily at {:02}:{:02}", hour, minute),
        }
    }
}

/// One scheduled scan task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleEntry {
    pub id: String,
    pub name: String,
    pub target: String,
    /// quick | full
    pub mode: String,
    pub rate: u32,
    pub timeout: u64,
    #[serde(default)]
    pub proxy: String,
    pub kind: ScheduleKind,
    pub enabled: bool,
    /// Directory where HTML/JSON/PDF reports are written
    pub reports_dir: String,
    #[serde(default)]
    pub last_run_ts: Option<i64>,
    #[serde(default)]
    pub next_run_ts: Option<i64>,
    /// "OK — 3 findings" or "ERROR: ..." from the last run
    #[serde(default)]
    pub last_status: Option<String>,
    #[serde(default)]
    pub runs: u64,
    #[serde(default)]
    pub created_ts: i64,
}

impl ScheduleEntry {
    pub fn new(name: String, target: String, kind: ScheduleKind) -> Self {
        Self {
            id: generate_id(),
            name,
            target,
            mode: "quick".into(),
            rate: 10,
            timeout: 8,
            proxy: String::new(),
            kind: kind.clone(),
            enabled: true,
            reports_dir: default_reports_dir().to_string_lossy().to_string(),
            last_run_ts: None,
            next_run_ts: Some(compute_next_ts(&kind, Local::now())),
            last_status: None,
            runs: 0,
            created_ts: Local::now().timestamp(),
        }
    }

    /// Recompute next_run from now
    pub fn reschedule(&mut self, now: DateTime<Local>) {
        self.next_run_ts = Some(compute_next_ts(&self.kind, now));
    }
}

/// Compute the next unix timestamp a schedule should fire (strictly after `now`)
pub fn compute_next_ts(kind: &ScheduleKind, now: DateTime<Local>) -> i64 {
    match kind {
        ScheduleKind::Interval { every_secs } => now.timestamp() + *every_secs as i64,
        ScheduleKind::Daily { hour, minute } => {
            let target_time = NaiveTime::from_hms_opt(*hour as u32, *minute as u32, 0)
                .unwrap_or(NaiveTime::from_hms_opt(3, 0, 0).unwrap());
            let today_at = now.date_naive().and_time(target_time);
            let today = today_at.and_local_timezone(now.timezone()).single();
            match today {
                Some(t) if t > now => t.timestamp(),
                _ => {
                    // tomorrow (handles month/year rollover via chrono)
                    let tomorrow = now.date_naive().succ_opt()
                        .map(|d| d.and_time(target_time))
                        .and_then(|ndt| ndt.and_local_timezone(now.timezone()).single());
                    match tomorrow {
                        Some(t) => t.timestamp(),
                        None => now.timestamp() + 86400, // extremely defensive fallback
                    }
                }
            }
        }
    }
}

/// Entries whose next_run is due (enabled + next_run_ts <= now)
pub fn due_entries(entries: &[ScheduleEntry], now_ts: i64) -> Vec<ScheduleEntry> {
    entries
        .iter()
        .filter(|e| e.enabled && e.next_run_ts.map(|t| t <= now_ts).unwrap_or(false))
        .cloned()
        .collect()
}

// ─── Persistence ───

pub fn entries_path() -> PathBuf {
    crate::utils::htool_dir().join("schedules.json")
}

pub fn load_entries() -> Vec<ScheduleEntry> {
    let path = entries_path();
    match std::fs::read_to_string(&path) {
        Ok(text) => serde_json::from_str(&text).unwrap_or_default(),
        Err(_) => Vec::new(),
    }
}

pub fn save_entries(entries: &[ScheduleEntry]) -> Result<(), String> {
    let path = entries_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("Cannot create config dir: {}", e))?;
    }
    let json = serde_json::to_string_pretty(entries).map_err(|e| e.to_string())?;
    std::fs::write(&path, json).map_err(|e| format!("Cannot save schedules: {}", e))
}

pub fn default_reports_dir() -> PathBuf {
    crate::utils::htool_dir().join("reports")
}

/// Short unique id for entries, e.g. "sc-8f3a1c"
pub fn generate_id() -> String {
    use rand::Rng;
    let n: u32 = rand::thread_rng().gen_range(0x100000..0xffffff);
    format!("sc-{:x}", n)
}

/// Parse "HH:MM" into (hour, minute)
pub fn parse_daily_time(s: &str) -> Result<(u8, u8), String> {
    let parts: Vec<&str> = s.trim().split(':').collect();
    if parts.len() != 2 {
        return Err(format!("Invalid time '{}'. Use HH:MM, e.g. 03:30", s));
    }
    let hour: u8 = parts[0].trim().parse().map_err(|_| format!("Invalid hour in '{}'", s))?;
    let minute: u8 = parts[1].trim().parse().map_err(|_| format!("Invalid minute in '{}'", s))?;
    if hour > 23 || minute > 59 {
        return Err(format!("Time '{}' out of range (00:00 – 23:59)", s));
    }
    Ok((hour, minute))
}

/// "in 2 h 15 m" style countdown from now to a unix ts
pub fn countdown(ts: i64) -> String {
    let secs = ts - Local::now().timestamp();
    if secs <= 0 { return "due now".into(); }
    if secs < 60 { return format!("in {}s", secs); }
    let mins = secs / 60;
    if mins < 60 { return format!("in {} m", mins); }
    let hours = mins / 60;
    if hours < 24 { return format!("in {} h {} m", hours, mins % 60); }
    format!("in {} d {} h", secs / 86400, (secs % 86400) / 3600)
}
