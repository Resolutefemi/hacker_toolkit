//! htool GUI — Modern Cyber Dashboard built with egui
//! Redesigned UI: dark neon theme, stat cards, badges, in-app report viewer
//! with a fully rendered report preview (HTML-style output, not source code).

use eframe::egui;
use egui::{RichText, Color32, Rounding, Stroke, Margin, ProgressBar, ScrollArea};
use htool::*;
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

// ══════════════════════════════════════════════════════════════════
//  Design System — palette, spacing, helpers
// ══════════════════════════════════════════════════════════════════

mod theme {
    use super::Color32;
    // Backgrounds (deep navy)
    pub const BG_DEEP:   Color32 = Color32::from_rgb(5, 8, 17);      // darkest — inputs
    pub const BG_MAIN:   Color32 = Color32::from_rgb(10, 14, 26);     // central panel
    pub const BG_PANEL:  Color32 = Color32::from_rgb(8, 11, 21);      // sidebar / top
    pub const BG_CARD:   Color32 = Color32::from_rgb(17, 23, 41);     // card fill
    pub const BG_ROW:    Color32 = Color32::from_rgb(23, 30, 52);     // subtle stripe
    pub const BG_WIDGET: Color32 = Color32::from_rgb(26, 34, 58);     // buttons / fields
    pub const STROKE:    Color32 = Color32::from_rgb(44, 56, 88);     // borders
    // Accent — neon emerald + cyan
    pub const ACCENT:    Color32 = Color32::from_rgb(0, 230, 158);
    pub const ACCENT_DIM: Color32 = Color32::from_rgba_premultiplied(0, 230, 158, 34);
    pub const CYAN:      Color32 = Color32::from_rgb(56, 189, 248);
    pub const PURPLE:    Color32 = Color32::from_rgb(167, 139, 250);
    // Text
    pub const TEXT:      Color32 = Color32::from_rgb(226, 232, 240);
    pub const TEXT_DIM:  Color32 = Color32::from_rgb(140, 152, 178);
    pub const TEXT_FAINT: Color32 = Color32::from_rgb(96, 106, 130);
    // Semantic
    pub const OK:        Color32 = Color32::from_rgb(52, 211, 153);
    pub const WARN:      Color32 = Color32::from_rgb(251, 191, 36);
    pub const DANGER:    Color32 = Color32::from_rgb(248, 113, 113);
    pub const ORANGE:    Color32 = Color32::from_rgb(251, 146, 60);
}

fn configure_theme(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    let v = &mut style.visuals;
    v.dark_mode = true;
    v.panel_fill = theme::BG_PANEL;
    v.window_fill = theme::BG_CARD;
    v.extreme_bg_color = theme::BG_DEEP;
    v.faint_bg_color = theme::BG_ROW;
    v.override_text_color = Some(theme::TEXT);
    v.button_frame = true;
    v.widgets.noninteractive.rounding = Rounding::same(6.0);
    v.widgets.inactive.bg_fill = theme::BG_WIDGET;
    v.widgets.inactive.weak_bg_fill = theme::BG_WIDGET;
    v.widgets.inactive.fg_stroke = Stroke::new(1.0_f32, theme::TEXT_DIM);
    v.widgets.inactive.bg_stroke = Stroke::new(1.0_f32, theme::STROKE);
    v.widgets.inactive.rounding = Rounding::same(8.0);
    v.widgets.hovered.bg_fill = Color32::from_rgb(32, 43, 72);
    v.widgets.hovered.weak_bg_fill = Color32::from_rgb(32, 43, 72);
    v.widgets.hovered.fg_stroke = Stroke::new(1.0_f32, theme::ACCENT);
    v.widgets.hovered.bg_stroke = Stroke::new(1.0_f32, theme::ACCENT.gamma_multiply(0.6));
    v.widgets.hovered.rounding = Rounding::same(8.0);
    v.widgets.active.bg_fill = theme::ACCENT_DIM;
    v.widgets.active.weak_bg_fill = theme::ACCENT_DIM;
    v.widgets.active.fg_stroke = Stroke::new(1.0_f32, theme::ACCENT);
    v.widgets.active.bg_stroke = Stroke::new(1.0_f32, theme::ACCENT);
    v.widgets.active.rounding = Rounding::same(8.0);
    v.selection.bg_fill = theme::ACCENT_DIM;
    v.selection.stroke = Stroke::new(1.0_f32, theme::ACCENT);
    ctx.set_style(style);

    let mut fonts = egui::FontDefinitions::default();
    fonts.families.entry(egui::FontFamily::Proportional).or_default().insert(0, "Ubuntu-Light".into());
    ctx.set_fonts(fonts);
}

/// A rounded card container with an optional accent title row
fn card(ui: &mut egui::Ui, title: &str, accent: Color32, add: impl FnOnce(&mut egui::Ui)) {
    egui::Frame::default()
        .fill(theme::BG_CARD)
        .rounding(Rounding::same(12.0))
        .stroke(Stroke::new(1.0_f32, theme::STROKE))
        .inner_margin(Margin::same(16.0))
        .outer_margin(egui::Margin { bottom: 12.0, ..Default::default() })
        .show(ui, |ui| {
            if !title.is_empty() {
                ui.horizontal(|ui| {
                    ui.painter().rect_filled(
                        egui::Rect::from_min_size(ui.cursor().min, egui::vec2(3.0, 18.0)),
                        Rounding::same(2.0),
                        accent,
                    );
                    ui.add_space(4.0);
                    ui.label(RichText::new(title).size(15.0).strong().color(theme::TEXT));
                });
                ui.add_space(8.0);
            }
            add(ui);
        });
}

/// A small rounded badge / chip
fn chip(ui: &mut egui::Ui, text: &str, fg: Color32, bg: Color32) {
    egui::Frame::default()
        .fill(bg)
        .rounding(Rounding::same(100.0))
        .inner_margin(Margin::symmetric(10.0, 3.0))
        .show(ui, |ui| {
            ui.label(RichText::new(text).size(12.0).color(fg).monospace());
        });
}

/// A big-number stat card used in the scan summary grid
fn stat_card(ui: &mut egui::Ui, count: usize, label: &str, color: Color32) {
    egui::Frame::default()
        .fill(theme::BG_ROW)
        .rounding(Rounding::same(10.0))
        .stroke(Stroke::new(1.0_f32, theme::STROKE))
        .inner_margin(Margin::same(10.0))
        .show(ui, |ui| {
            ui.with_layout(egui::Layout::top_down_justified(egui::Align::Center), |ui| {
                ui.label(RichText::new(count.to_string()).size(24.0).strong().color(color));
                ui.label(RichText::new(label).size(10.5).color(theme::TEXT_DIM));
            });
        });
}

/// Severity banner color for a given severity string
fn severity_color(sev: &str) -> Color32 {
    match sev {
        "CRITICAL" => theme::DANGER,
        "HIGH" => theme::ORANGE,
        "MEDIUM" => theme::WARN,
        "LOW" => theme::CYAN,
        _ => theme::OK,
    }
}

fn tech_color(t: &str) -> Color32 {
    if t.starts_with("[WAF]") { theme::ORANGE } else { theme::CYAN }
}

fn field_label(ui: &mut egui::Ui, text: &str) {
    ui.label(RichText::new(text).size(12.5).color(theme::TEXT_DIM));
}

fn primary_button(ui: &mut egui::Ui, text: &str) -> bool {
    let resp = ui.add_sized([160.0, 34.0],
        egui::Button::new(RichText::new(text).size(14.5).strong().color(Color32::BLACK))
            .fill(theme::ACCENT)
            .rounding(Rounding::same(8.0)));
    resp.clicked()
}

fn ghost_button(ui: &mut egui::Ui, text: &str) -> bool {
    ui.add(egui::Button::new(RichText::new(text).size(12.5).color(theme::CYAN))
        .fill(Color32::TRANSPARENT)
        .stroke(Stroke::new(1.0_f32, theme::STROKE))
        .rounding(Rounding::same(8.0))).clicked()
}

/// Render a mono code-style block (headers, SSL info, JSON…)
fn code_block(ui: &mut egui::Ui, id: &str, content: &str, max_h: f32) {
    ScrollArea::vertical().max_height(max_h).id_source(("codeblk_", id)).show(ui, |ui| {
        egui::Frame::default()
            .fill(theme::BG_DEEP)
            .rounding(Rounding::same(8.0))
            .inner_margin(Margin::same(10.0))
            .show(ui, |ui| {
                ui.set_min_width(ui.available_width());
                ui.label(RichText::new(content).size(11.5).color(theme::TEXT_DIM).monospace());
            });
    });
}

/// Syntax-tinted JSON viewer (keys cyan, strings green, numbers orange)
fn json_viewer(ui: &mut egui::Ui, json: &str, max_h: f32) {
    ScrollArea::vertical().max_height(max_h).id_source("json_viewer").show(ui, |ui| {
        egui::Frame::default()
            .fill(theme::BG_DEEP)
            .rounding(Rounding::same(8.0))
            .inner_margin(Margin::same(10.0))
            .show(ui, |ui| {
                ui.set_min_width(ui.available_width());
                for line in json.lines() {
                    let trimmed = line.trim_start();
                    let indent_ws = &line[..line.len() - trimmed.len()];
                    let (key_part, rest) = match trimmed.split_once("\": ") {
                        Some((k, r)) => (format!("{}\"", k), r),
                        None => (String::new(), trimmed),
                    };
                    ui.horizontal_wrapped(|ui| {
                        ui.label(RichText::new(indent_ws.to_string() + &key_part).size(11.5).color(theme::CYAN).monospace());
                        if !rest.is_empty() && !key_part.is_empty() {
                            let color = if rest.starts_with('"') { theme::OK } else if rest.chars().next().is_some_and(|c| c.is_ascii_digit()) { theme::ORANGE } else { theme::TEXT_DIM };
                            ui.label(RichText::new(rest).size(11.5).color(color).monospace());
                        } else if !rest.is_empty() {
                            ui.label(RichText::new(rest).size(11.5).color(theme::TEXT_DIM).monospace());
                        }
                    });
                }
            });
    });
}

/// List of vulnerable URLs with warning bullets
fn vuln_list(ui: &mut egui::Ui, items: &[String], color: Color32) {
    if items.is_empty() {
        chip(ui, "✓ none found", theme::OK, theme::OK.gamma_multiply(0.13));
        return;
    }
    for item in items {
        egui::Frame::default()
            .fill(theme::BG_DEEP)
            .rounding(Rounding::same(6.0))
            .inner_margin(Margin::symmetric(10.0, 6.0))
            .outer_margin(egui::Margin { bottom: 4.0, ..Default::default() })
            .show(ui, |ui| {
                ui.set_min_width(ui.available_width());
                ui.label(RichText::new(format!("⚠  {}", item)).size(12.0).color(color).monospace());
            });
    }
}

/// Grid of chips
fn chip_grid(ui: &mut egui::Ui, items: &[String], default_fg: Color32) {
    if items.is_empty() {
        ui.label(RichText::new("— nothing detected —").size(12.0).color(theme::TEXT_FAINT));
        return;
    }
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing = egui::vec2(6.0, 6.0);
        for item in items {
            let fg = tech_color(item);
            chip(ui, item, if fg == default_fg { default_fg } else { fg }, fg.gamma_multiply(0.12));
        }
    });
}

// ══════════════════════════════════════════════════════════════════
//  App state
// ══════════════════════════════════════════════════════════════════

enum AppMessage {
    ScanProgress(f32),
    ScanFinished(Box<ScanResult>),
    StressFinished(Result<u64, String>),
    CredStuffFinished(Vec<LoginResult>),
    SpamFinished(usize),
}

#[derive(PartialEq, Clone, Copy)]
enum ActiveTab {
    Dashboard,
    Scanner,
    Stress,
    CredStuff,
    Spam,
    Payload,
    ReportViewer,
    CveSearch,
}

impl ActiveTab {
    fn all() -> [(ActiveTab, &'static str, &'static str); 8] {
        [
            (ActiveTab::Dashboard,    "◈", "Dashboard"),
            (ActiveTab::Scanner,      "◉", "Scanner"),
            (ActiveTab::Stress,       "⚡", "Stress Test"),
            (ActiveTab::CredStuff,    "@", "Cred Stuffing"),
            (ActiveTab::Spam,         "✉", "Spam & Flood"),
            (ActiveTab::Payload,      ">_", "Payload Gen"),
            (ActiveTab::ReportViewer, "▤", "Report Viewer"),
            (ActiveTab::CveSearch,    "☰", "CVE Database"),
        ]
    }
    fn title(&self) -> &'static str {
        Self::all().iter().find(|(t, _, _)| t == self).map(|(_, _, n)| *n).unwrap_or("")
    }
}

struct UltimateApp {
    active_tab: ActiveTab,
    // Scanner
    scan_target: String,
    scan_mode: String,
    scan_rate: u32,
    scan_proxy: String,
    scan_wordlist_path: String,
    scan_timeout: u64,
    scan_result: Option<ScanResult>,
    scan_in_progress: bool,
    scan_progress: f32,
    scan_phase: String,
    // Stress
    stress_target: String,
    stress_attack: String,
    stress_threads: usize,
    stress_duration: u64,
    stress_proxy: String,
    stress_result: Option<String>,
    stress_in_progress: bool,
    // Credential Stuffing
    cred_login_url: String,
    cred_user_field: String,
    cred_pass_field: String,
    cred_users_path: String,
    cred_passes_path: String,
    cred_threads: usize,
    cred_proxy_list: String,
    cred_success_text: String,
    cred_results: Vec<LoginResult>,
    cred_in_progress: bool,
    // Spam
    spam_endpoint: String,
    spam_kind: String,
    spam_count: usize,
    spam_threads: usize,
    spam_proxy: String,
    spam_rate: u32,
    spam_result: Option<usize>,
    spam_in_progress: bool,
    // Payload
    payload_type: String,
    payload_platform: String,
    payload_lhost: String,
    payload_lport: u16,
    payload_url: String,
    payload_password: String,
    payload_generated: String,
    // Report viewer
    rv_json_path: String,
    rv_result: Option<ScanResult>,
    rv_error: Option<String>,
    rv_show_json: bool,
    rv_json_text: String,
    // CVE
    cve_query: String,
    cve_results: Vec<CveEntry>,
    // Misc
    logs: Vec<String>,
    tx: std::sync::mpsc::Sender<AppMessage>,
    rx: std::sync::mpsc::Receiver<AppMessage>,
    show_about: bool,
}

impl Default for UltimateApp {
    fn default() -> Self {
        let (tx_chan, rx_chan) = std::sync::mpsc::channel();
        Self {
            active_tab: ActiveTab::Dashboard,
            scan_target: String::new(),
            scan_mode: "quick".to_string(),
            scan_rate: 10,
            scan_proxy: String::new(),
            scan_wordlist_path: String::new(),
            scan_timeout: 8,
            scan_result: None,
            scan_in_progress: false,
            scan_progress: 0.0,
            scan_phase: String::new(),
            stress_target: String::new(),
            stress_attack: "http".to_string(),
            stress_threads: 100,
            stress_duration: 30,
            stress_proxy: String::new(),
            stress_result: None,
            stress_in_progress: false,
            cred_login_url: String::new(),
            cred_user_field: "username".to_string(),
            cred_pass_field: "password".to_string(),
            cred_users_path: String::new(),
            cred_passes_path: String::new(),
            cred_threads: 10,
            cred_proxy_list: String::new(),
            cred_success_text: "dashboard".to_string(),
            cred_results: Vec::new(),
            cred_in_progress: false,
            spam_endpoint: String::new(),
            spam_kind: "db-flood".to_string(),
            spam_count: 100,
            spam_threads: 10,
            spam_proxy: String::new(),
            spam_rate: 20,
            spam_result: None,
            spam_in_progress: false,
            payload_type: "reverse".to_string(),
            payload_platform: "linux".to_string(),
            payload_lhost: String::new(),
            payload_lport: 4444,
            payload_url: String::new(),
            payload_password: String::new(),
            payload_generated: String::new(),
            rv_json_path: String::new(),
            rv_result: None,
            rv_error: None,
            rv_show_json: false,
            rv_json_text: String::new(),
            cve_query: String::new(),
            cve_results: Vec::new(),
            logs: vec![format!("[{}] htool v{} ready. Select a module to begin.", chrono::Local::now().format("%H:%M:%S"), env!("CARGO_PKG_VERSION"))],
            tx: tx_chan,
            rx: rx_chan,
            show_about: false,
        }
    }
}

impl UltimateApp {
    fn add_log(&mut self, msg: String) {
        self.logs.push(format!("[{}] {}", chrono::Local::now().format("%H:%M:%S"), msg));
        if self.logs.len() > 200 {
            self.logs.remove(0);
        }
    }

    fn load_scan_json(&mut self) {
        self.rv_error = None;
        self.rv_result = None;
        self.rv_json_text.clear();
        match std::fs::read_to_string(&self.rv_json_path) {
            Ok(text) => match serde_json::from_str::<ScanResult>(&text) {
                Ok(res) => {
                    self.rv_json_text = serde_json::to_string_pretty(&res).unwrap_or_default();
                    self.add_log(format!("Loaded report: {} ({} findings)", self.rv_json_path, res.total_findings()));
                    self.rv_result = Some(res);
                }
                Err(e) => {
                    self.rv_error = Some(format!("Invalid scan JSON: {}", e));
                    self.add_log("Failed to parse report JSON.".to_string());
                }
            },
            Err(e) => {
                self.rv_error = Some(format!("Cannot read file: {}", e));
                self.add_log("Cannot read report file.".to_string());
            }
        }
    }

    /// Push the current scan result into the report viewer
    fn view_result_in_report(&mut self, res: &ScanResult) {
        self.rv_result = Some(res.clone());
        self.rv_json_text = serde_json::to_string_pretty(res).unwrap_or_default();
        self.rv_json_path = "(current session)".to_string();
        self.rv_error = None;
        self.rv_show_json = false;
        self.active_tab = ActiveTab::ReportViewer;
    }

    fn run_scan(&mut self, ctx: &egui::Context) {
        let target = self.scan_target.clone();
        let mode = self.scan_mode.clone();
        let rate = self.scan_rate;
        let proxy = if self.scan_proxy.is_empty() { None } else { Some(self.scan_proxy.clone()) };
        let wordlist = if !self.scan_wordlist_path.is_empty() {
            load_wordlist(Some(&self.scan_wordlist_path))
        } else {
            load_wordlist(None)
        };
        let timeout = self.scan_timeout;
        let scan_type = if mode == "full" { ScanType::Full } else { ScanType::Quick };
        let config = ScannerConfig {
            scan_type,
            rate_limit_rps: rate,
            proxy,
            wordlist,
            timeout_secs: timeout,
            user_agent: utils::random_user_agent(),
        };
        self.scan_in_progress = true;
        self.scan_progress = 0.0;
        self.scan_phase = "Initialising…".to_string();
        let ctx_clone = ctx.clone();
        let target_clone = target.clone();
        let tx = self.tx.clone();
        tokio::spawn(async move {
            let tx_progress = tx.clone();
            let ctx_progress = ctx_clone.clone();
            let result = run_full_scan(target_clone, config, Some(Box::new(move |progress| {
                let _ = tx_progress.send(AppMessage::ScanProgress(progress));
                ctx_progress.request_repaint();
            }))).await;
            let _ = tx.send(AppMessage::ScanFinished(Box::new(result)));
            ctx_clone.request_repaint();
        });
        self.add_log(format!("Scan started on {} ({} mode)", target, mode));
    }

    fn phase_label(progress: f32) -> &'static str {
        if progress < 0.001 { "Initialising…" }
        else if progress < 0.2 { "Port scanning…" }
        else if progress < 0.4 { "Directory brute-force…" }
        else if progress < 0.6 { "SQL injection tests…" }
        else if progress < 0.7 { "XSS tests…" }
        else if progress < 0.8 { "Subdomain enumeration…" }
        else { "Fingerprinting & analysis…" }
    }
}

// ══════════════════════════════════════════════════════════════════
//  UI implementation
// ══════════════════════════════════════════════════════════════════

impl eframe::App for UltimateApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        configure_theme(ctx);

        // Drain messages from background tasks
        while let Ok(msg) = self.rx.try_recv() {
            match msg {
                AppMessage::ScanProgress(p) => {
                    self.scan_progress = p;
                    self.scan_phase = Self::phase_label(p).to_string();
                }
                AppMessage::ScanFinished(res) => {
                    self.scan_in_progress = false;
                    self.scan_progress = 1.0;
                    self.scan_phase = "Complete".to_string();
                    self.add_log(format!(
                        "Scan completed — {} findings across {} open ports.",
                        res.total_findings(), res.open_ports.len()
                    ));
                    self.scan_result = Some(*res);
                }
                AppMessage::StressFinished(res) => {
                    self.stress_in_progress = false;
                    match res {
                        Ok(sent) => {
                            self.stress_result = Some(format!("✓ Completed — {} requests/connections sent", sent));
                            self.add_log(format!("Stress test completed. Sent {} requests.", sent));
                        }
                        Err(e) => {
                            self.stress_result = Some(format!("✗ Failed: {}", e));
                            self.add_log(format!("Stress test failed: {}", e));
                        }
                    }
                }
                AppMessage::CredStuffFinished(results) => {
                    self.cred_in_progress = false;
                    let successful = results.iter().filter(|r| r.success).count();
                    self.add_log(format!("Credential stuffing finished. Successful: {}/{}", successful, results.len()));
                    self.cred_results = results;
                }
                AppMessage::SpamFinished(sent) => {
                    self.spam_in_progress = false;
                    self.spam_result = Some(sent);
                    self.add_log(format!("Spam module finished. Sent {} requests.", sent));
                }
            }
        }

        self.draw_topbar(ctx);
        self.draw_sidebar(ctx);

        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(theme::BG_MAIN).inner_margin(Margin::same(18.0)))
            .show(ctx, |ui| {
                match self.active_tab {
                    ActiveTab::Dashboard    => self.tab_dashboard(ui),
                    ActiveTab::Scanner      => self.tab_scanner(ui, ctx),
                    ActiveTab::Stress       => self.tab_stress(ui, ctx),
                    ActiveTab::CredStuff    => self.tab_credstuff(ui, ctx),
                    ActiveTab::Spam         => self.tab_spam(ui, ctx),
                    ActiveTab::Payload      => self.tab_payload(ui),
                    ActiveTab::ReportViewer => self.tab_report(ui),
                    ActiveTab::CveSearch    => self.tab_cve(ui),
                }
                ui.add_space(6.0);
                self.draw_logstrip(ui);
            });

        // About modal
        if self.show_about {
            egui::Window::new(RichText::new("About htool").strong())
                .collapsible(false).resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    ui.label(RichText::new("htool").size(26.0).strong().color(theme::ACCENT));
                    ui.label(RichText::new(format!("v{} — Ultimate Hacker Toolkit", env!("CARGO_PKG_VERSION"))).color(theme::TEXT_DIM));
                    ui.add_space(6.0);
                    ui.label("A multi-threaded networking utility and vulnerability scanner.");
                    ui.label(RichText::new("Authorised security testing only.").color(theme::WARN).size(12.0));
                    ui.add_space(8.0);
                    if ui.button("Close").clicked() { self.show_about = false; }
                });
        }
    }
}

impl UltimateApp {
    // ── Top bar ────────────────────────────────────────────────
    fn draw_topbar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("topbar")
            .frame(egui::Frame::default().fill(theme::BG_PANEL).inner_margin(Margin::symmetric(16.0, 10.0)))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("htool").size(21.0).strong().color(theme::ACCENT).monospace());
                    ui.label(RichText::new(format!("v{}", env!("CARGO_PKG_VERSION"))).size(10.5).color(theme::TEXT_FAINT).monospace());
                    ui.add_space(8.0);
                    chip(ui, &format!("MODULE  ·  {}", self.active_tab.title()), theme::ACCENT, theme::ACCENT_DIM);
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let busy = self.scan_in_progress || self.stress_in_progress || self.cred_in_progress || self.spam_in_progress;
                        if busy {
                            ui.spinner();
                            ui.label(RichText::new("running…").size(12.0).color(theme::WARN));
                        } else {
                            ui.label(RichText::new("● idle").size(12.0).color(theme::OK));
                        }
                        if ui.add(egui::Button::new(RichText::new("About").size(12.0).color(theme::TEXT_DIM)).fill(Color32::TRANSPARENT)).clicked() {
                            self.show_about = true;
                        }
                    });
                });
            });
    }

    // ── Sidebar ────────────────────────────────────────────────
    fn draw_sidebar(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("sidebar")
            .exact_width(190.0)
            .frame(egui::Frame::default().fill(theme::BG_PANEL).inner_margin(Margin { left: 10.0, right: 10.0, top: 14.0, bottom: 10.0 }))
            .show(ctx, |ui| {
                ui.label(RichText::new("MODULES").size(10.0).color(theme::TEXT_FAINT).monospace());
                ui.add_space(8.0);
                for (tab, icon, name) in ActiveTab::all() {
                    let selected = self.active_tab == tab;
                    let resp = egui::Frame::default()
                        .fill(if selected { theme::ACCENT_DIM } else { Color32::TRANSPARENT })
                        .rounding(Rounding::same(8.0))
                        .inner_margin(Margin::symmetric(10.0, 7.0))
                        .show(ui, |ui| {
                            ui.set_min_width(ui.available_width());
                            ui.horizontal(|ui| {
                                if selected {
                                    ui.painter().rect_filled(
                                        egui::Rect::from_min_size(ui.cursor().min, egui::vec2(3.0, 16.0)),
                                        Rounding::same(2.0), theme::ACCENT);
                                }
                                ui.label(RichText::new(icon).size(13.0).color(if selected { theme::ACCENT } else { theme::TEXT_DIM }));
                                ui.label(RichText::new(name).size(13.0).color(if selected { theme::TEXT } else { theme::TEXT_DIM }).strong());
                            });
                        })
                        .response
                        .interact(egui::Sense::click());
                    if resp.clicked() { self.active_tab = tab; }
                    if resp.hovered() { ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand); }
                }

                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    ui.add_space(8.0);
                    ui.separator();
                    ui.add_space(6.0);
                    ui.label(RichText::new("by Resolute Femi").size(10.5).color(theme::TEXT_FAINT).monospace());
                    ui.label(RichText::new("authorised use only").size(10.0).color(theme::WARN.gamma_multiply(0.8)));
                });
            });
    }

    // ── Log strip (bottom of every tab) ───────────────────────
    fn draw_logstrip(&mut self, ui: &mut egui::Ui) {
        ui.separator();
        ui.horizontal(|ui| {
            ui.label(RichText::new("ACTIVITY LOG").size(10.0).color(theme::TEXT_FAINT).monospace());
            if ui.add(egui::Button::new(RichText::new("clear").size(10.5).color(theme::TEXT_FAINT)).fill(Color32::TRANSPARENT)).clicked() {
                self.logs.clear();
            }
        });
        ScrollArea::vertical().max_height(90.0).stick_to_bottom(true).id_source("logstrip").show(ui, |ui| {
            egui::Frame::default().fill(theme::BG_DEEP).rounding(Rounding::same(8.0))
                .inner_margin(Margin::same(8.0)).show(ui, |ui| {
                    ui.set_min_width(ui.available_width());
                    for log in self.logs.iter().rev() {
                        ui.label(RichText::new(log).size(11.0).color(theme::TEXT_FAINT).monospace());
                    }
                });
        });
    }

    // ── Dashboard ──────────────────────────────────────────────
    fn tab_dashboard(&mut self, ui: &mut egui::Ui) {
        ui.label(RichText::new("Command Center").size(24.0).strong().color(theme::TEXT));
        ui.label(RichText::new("Select a module below to begin your authorised assessment.").size(13.0).color(theme::TEXT_DIM));
        ui.add_space(12.0);

        // Severity overview of the latest scan
        if let Some(res) = self.scan_result.clone() {
            let sev = res.severity();
            egui::Frame::default()
                .fill(severity_color(sev).gamma_multiply(0.10))
                .rounding(Rounding::same(10.0))
                .stroke(Stroke::new(1.0_f32, severity_color(sev).gamma_multiply(0.5)))
                .inner_margin(Margin::same(12.0))
                .outer_margin(egui::Margin { bottom: 12.0, ..Default::default() })
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(format!("LAST SCAN — {}", res.target)).size(13.0).strong().color(severity_color(sev)));
                        chip(ui, sev, severity_color(sev), severity_color(sev).gamma_multiply(0.15));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.add(egui::Button::new(RichText::new("View full report ▸").size(12.0).color(theme::CYAN)).fill(Color32::TRANSPARENT)).clicked() {
                                self.view_result_in_report(&res);
                            }
                        });
                    });
                    ui.label(RichText::new(format!("{} security findings · {} open ports · {} technologies", res.total_findings(), res.open_ports.len(), res.technologies.len())).size(12.0).color(theme::TEXT_DIM));
                });
        }

        // Module launcher grid
        let modules: Vec<(&str, &str, &str, ActiveTab, Color32)> = vec![
            ("◉", "Vulnerability Scanner", "Ports · SQLi · XSS · dirs · subdomains · SSL · headers · CVEs", ActiveTab::Scanner, theme::ACCENT),
            ("⚡", "Stress Testing", "HTTP / Slowloris / UDP / SYN load simulation (authorised only)", ActiveTab::Stress, theme::ORANGE),
            ("@", "Credential Stuffing", "Mass login tests with wordlists, proxies & rate limiting", ActiveTab::CredStuff, theme::CYAN),
            ("✉", "Spam & Flood", "DB flood · comment spam · registration spam rate-limit tests", ActiveTab::Spam, theme::WARN),
            (">_", "Payload Generator", "Reverse & bind shells · web shells · download & exec", ActiveTab::Payload, theme::PURPLE),
            ("▤", "Report Viewer", "Open scan JSON — view the rendered report & JSON in-app", ActiveTab::ReportViewer, theme::OK),
            ("☰", "CVE Database", "Offline CVE lookup by product, keyword or year", ActiveTab::CveSearch, theme::DANGER),
        ];
        let cells = modules.len();
        let cols = 3usize;
        egui::Grid::new("module_grid").min_col_width(210.0).spacing([10.0, 10.0]).show(ui, |ui| {
            for (i, (icon, name, desc, tab, color)) in modules.iter().enumerate() {
                egui::Frame::default()
                    .fill(theme::BG_CARD).rounding(Rounding::same(12.0))
                    .stroke(Stroke::new(1.0_f32, theme::STROKE))
                    .inner_margin(Margin::same(14.0))
                    .show(ui, |ui| {
                        ui.set_min_width(190.0);
                        ui.set_min_height(84.0);
                        ui.vertical(|ui| {
                            ui.horizontal(|ui| {
                                ui.label(RichText::new(*icon).size(17.0).color(*color));
                                ui.label(RichText::new(*name).size(14.0).strong().color(theme::TEXT));
                            });
                            ui.add_space(4.0);
                            ui.label(RichText::new(*desc).size(11.0).color(theme::TEXT_DIM));
                            ui.add_space(6.0);
                            let btn = ui.add(egui::Button::new(RichText::new(format!("Open {}", *icon)).size(11.5).color(*color)).fill(Color32::TRANSPARENT).stroke(Stroke::new(1.0_f32, theme::STROKE)));
                            if btn.clicked() { self.active_tab = *tab; }
                        });
                    });
                if (i + 1) % cols == 0 && i + 1 < cells { ui.end_row(); }
                if i + 1 == cells { ui.end_row(); }
            }
        });
        ui.add_space(4.0);
        ui.label(RichText::new(format!("⚠ For authorised security testing and educational purposes only. v{}", env!("CARGO_PKG_VERSION"))).size(11.0).color(theme::TEXT_FAINT));
    }

    // ── Scanner ────────────────────────────────────────────────
    fn tab_scanner(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.label(RichText::new("Vulnerability Scanner").size(22.0).strong().color(theme::TEXT));
        ui.add_space(8.0);

        card(ui, "Scan Configuration", theme::ACCENT, |ui| {
            ui.horizontal(|ui| {
                field_label(ui, "TARGET");
                ui.add(egui::TextEdit::singleline(&mut self.scan_target).hint_text("https://example.com or 10.0.0.1").desired_width(380.0));
                ui.add_space(12.0);
                field_label(ui, "MODE");
                let mk = |ui: &mut egui::Ui, label: &str, val: &str| {
                    let sel = self.scan_mode == val;
                    let _ = ui.selectable_label(sel, RichText::new(label).color(if sel { theme::ACCENT } else { theme::TEXT_DIM }).strong());
                };
                mk(ui, "⚡ Quick", "quick");
                mk(ui, "🐢 Full (1–1024)", "full");
            });
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                field_label(ui, "RATE (RPS)");
                ui.add(egui::DragValue::new(&mut self.scan_rate).clamp_range(1..=200));
                ui.add_space(14.0);
                field_label(ui, "TIMEOUT (s)");
                ui.add(egui::DragValue::new(&mut self.scan_timeout).clamp_range(1..=30));
                ui.add_space(14.0);
                field_label(ui, "PROXY");
                ui.add(egui::TextEdit::singleline(&mut self.scan_proxy).hint_text("http://127.0.0.1:8080").desired_width(200.0));
                ui.add_space(14.0);
                field_label(ui, "WORDLIST");
                ui.add(egui::TextEdit::singleline(&mut self.scan_wordlist_path).desired_width(170.0));
                if ui.button("📂").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        self.scan_wordlist_path = path.display().to_string();
                    }
                }
            });
        });

        ui.horizontal(|ui| {
            if !self.scan_in_progress {
                if primary_button(ui, "▶  START SCAN") && !self.scan_target.is_empty() {
                    self.run_scan(ctx);
                }
            } else {
                ui.add(egui::Button::new(RichText::new("● Scanning…").size(14.5).strong().color(theme::TEXT_DIM)).fill(theme::BG_WIDGET).rounding(Rounding::same(8.0)).min_size(egui::vec2(160.0, 34.0)));
            }
            ui.add_space(10.0);
            if ghost_button(ui, "Copy JSON to clipboard") {
                if let Some(res) = &self.scan_result {
                    if let Ok(json) = serde_json::to_string_pretty(res) {
                        ui.output_mut(|o| o.copied_text = json);
                        self.add_log("Report JSON copied to clipboard.".to_string());
                    }
                }
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                chip(ui, "authorised targets only", theme::WARN, theme::WARN.gamma_multiply(0.10));
            });
        });
        ui.add_space(10.0);
        if self.scan_in_progress {
            card(ui, "Progress", theme::CYAN, |ui| {
                ui.add(ProgressBar::new(self.scan_progress).text(format!("{}  ·  {:.0}%", self.scan_phase, self.scan_progress * 100.0)).fill(theme::ACCENT));
                ui.add_space(4.0);
                ui.label(RichText::new("Multi-threaded async scan in progress — you can browse other modules.").size(11.5).color(theme::TEXT_FAINT));
            });
        }

        if let Some(result) = self.scan_result.clone() {
            ScrollArea::vertical().id_source("scan_results").show(ui, |ui| {
                self.render_report_preview(ui, &result);
            });
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                if ui.button("▤  View in Report Viewer").clicked() { self.view_result_in_report(&result); }
                if ui.button("📄 Export HTML").clicked() {
                    if let Some(path) = rfd::FileDialog::new().add_filter("HTML Report", &["html"]).set_file_name("htool_report.html").save_file() {
                        let p = path.display().to_string();
                        match save_html_report(&result, &p) {
                            Ok(_) => self.add_log(format!("HTML report exported to {}", p)),
                            Err(e) => self.add_log(format!("Export failed: {}", e)),
                        }
                    }
                }
                if ui.button("💾 Export JSON").clicked() {
                    if let Some(path) = rfd::FileDialog::new().add_filter("JSON Report", &["json"]).set_file_name("htool_report.json").save_file() {
                        let p = path.display().to_string();
                        match save_json_report(&result, &p) {
                            Ok(_) => self.add_log(format!("JSON report exported to {}", p)),
                            Err(e) => self.add_log(format!("Export failed: {}", e)),
                        }
                    }
                }
                if ui.button("🌐 Open report in browser").clicked() {
                    let tmp = std::env::temp_dir().join("htool_report_preview.html");
                    let _ = save_html_report(&result, tmp.to_string_lossy().as_ref());
                    let _ = open::that(tmp.to_string_lossy().to_string());
                }
            });
        }
    }

    // ── Stress ─────────────────────────────────────────────────
    fn tab_stress(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.label(RichText::new("Stress Testing").size(22.0).strong().color(theme::TEXT));
        ui.add_space(8.0);
        card(ui, "Attack Simulation (authorised targets only)", theme::ORANGE, |ui| {
            ui.horizontal(|ui| {
                field_label(ui, "TARGET");
                ui.add(egui::TextEdit::singleline(&mut self.stress_target).hint_text("http://192.168.1.1:80").desired_width(330.0));
                ui.add_space(12.0);
                field_label(ui, "TYPE");
                egui::ComboBox::from_id_source("stress_attack_combo").selected_text(&self.stress_attack).width(130.0).show_ui(ui, |ui| {
                    for attack in &["http", "http-random", "slowloris", "udp", "syn", "advanced", "icmp"] {
                        ui.selectable_value(&mut self.stress_attack, attack.to_string(), *attack);
                    }
                });
            });
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                field_label(ui, "THREADS");
                ui.add(egui::DragValue::new(&mut self.stress_threads).clamp_range(1..=1000));
                ui.add_space(14.0);
                field_label(ui, "DURATION (s)");
                ui.add(egui::DragValue::new(&mut self.stress_duration).clamp_range(1..=3600));
                ui.add_space(14.0);
                field_label(ui, "PROXY");
                ui.add(egui::TextEdit::singleline(&mut self.stress_proxy).hint_text("optional").desired_width(220.0));
            });
        });
        ui.horizontal(|ui| {
            if !self.stress_in_progress {
                if ui.add(egui::Button::new(RichText::new("⚡  LAUNCH TEST").strong().color(Color32::BLACK)).fill(theme::ORANGE).rounding(Rounding::same(8.0)).min_size(egui::vec2(160.0, 34.0))).clicked() && !self.stress_target.is_empty() {
                    self.stress_in_progress = true;
                    self.stress_result = None;
                    let target = self.stress_target.clone();
                    let attack = self.stress_attack.clone();
                    let threads = self.stress_threads;
                    let duration = self.stress_duration;
                    let proxy = if self.stress_proxy.is_empty() { None } else { Some(self.stress_proxy.clone()) };
                    let tx = self.tx.clone();
                    let ctx_clone = ctx.clone();
                    self.add_log(format!("Stress test launched: {} ({})", target, attack));
                    tokio::spawn(async move {
                        let result = stress::launch_stress_test(&target, &attack, threads, duration, proxy.as_deref()).await;
                        let _ = tx.send(AppMessage::StressFinished(result));
                        ctx_clone.request_repaint();
                    });
                }
            } else {
                ui.spinner();
                ui.label(RichText::new("Test running…").color(theme::WARN));
            }
            if ui.button("Stop waiting & reset").clicked() && !self.stress_in_progress { self.stress_result = None; }
        });
        if let Some(res_str) = &self.stress_result {
            ui.add_space(8.0);
            let ok = res_str.starts_with('✓');
            egui::Frame::default().fill(if ok { theme::OK.gamma_multiply(0.1) } else { theme::DANGER.gamma_multiply(0.1) })
                .rounding(Rounding::same(8.0)).inner_margin(Margin::same(10.0)).show(ui, |ui| {
                    ui.label(RichText::new(res_str).color(if ok { theme::OK } else { theme::DANGER }).size(13.0));
                });
        }
    }

    // ── Credential Stuffing ────────────────────────────────────
    fn tab_credstuff(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.label(RichText::new("Credential Stuffing").size(22.0).strong().color(theme::TEXT));
        ui.add_space(8.0);
        card(ui, "Login Test Configuration", theme::CYAN, |ui| {
            ui.horizontal(|ui| {
                field_label(ui, "LOGIN URL");
                ui.add(egui::TextEdit::singleline(&mut self.cred_login_url).hint_text("https://example.com/login").desired_width(300.0));
                ui.add_space(10.0);
                field_label(ui, "USER FIELD");
                ui.add(egui::TextEdit::singleline(&mut self.cred_user_field).desired_width(90.0));
                ui.add_space(10.0);
                field_label(ui, "PASS FIELD");
                ui.add(egui::TextEdit::singleline(&mut self.cred_pass_field).desired_width(90.0));
            });
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                field_label(ui, "USERS FILE");
                ui.add(egui::TextEdit::singleline(&mut self.cred_users_path).desired_width(170.0));
                if ui.button("📂").clicked() {
                    if let Some(p) = rfd::FileDialog::new().pick_file() { self.cred_users_path = p.display().to_string(); }
                }
                ui.add_space(10.0);
                field_label(ui, "PASSWORDS FILE");
                ui.add(egui::TextEdit::singleline(&mut self.cred_passes_path).desired_width(170.0));
                if ui.button("📂").clicked() {
                    if let Some(p) = rfd::FileDialog::new().pick_file() { self.cred_passes_path = p.display().to_string(); }
                }
            });
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                field_label(ui, "THREADS");
                ui.add(egui::DragValue::new(&mut self.cred_threads).clamp_range(1..=100));
                ui.add_space(14.0);
                field_label(ui, "SUCCESS TEXT");
                ui.add(egui::TextEdit::singleline(&mut self.cred_success_text).hint_text("text shown after valid login").desired_width(180.0));
                ui.add_space(14.0);
                field_label(ui, "PROXY LIST FILE");
                ui.add(egui::TextEdit::singleline(&mut self.cred_proxy_list).hint_text("optional").desired_width(150.0));
            });
        });
        ui.horizontal(|ui| {
            if ui.add(egui::Button::new(RichText::new("▶  START STUFFING").strong().color(Color32::BLACK)).fill(theme::CYAN).rounding(Rounding::same(8.0)).min_size(egui::vec2(160.0, 34.0))).clicked()
                && !self.cred_in_progress && !self.cred_login_url.is_empty() {
                self.cred_in_progress = true;
                self.cred_results.clear();
                let usernames = load_wordlist_from_file(&self.cred_users_path);
                let passwords = load_wordlist_from_file(&self.cred_passes_path);
                self.add_log(format!("Credential stuffing started — {}×{} combos", usernames.len(), passwords.len()));
                let config = CredStuffConfig {
                    login_url: self.cred_login_url.clone(),
                    username_field: self.cred_user_field.clone(),
                    password_field: self.cred_pass_field.clone(),
                    extra_fields: vec![],
                    success_indicator: Some(self.cred_success_text.clone()),
                    failure_indicator: None,
                    threads: self.cred_threads,
                    proxy_list: if self.cred_proxy_list.is_empty() { None } else { Some(load_proxy_list(&self.cred_proxy_list)) },
                    rate_limit_rps: 10,
                    timeout_secs: 10,
                    user_agent: utils::random_user_agent(),
                };
                let tx = self.tx.clone();
                let ctx_clone = ctx.clone();
                tokio::spawn(async move {
                    let results = credential_stuffing(&config, usernames, passwords, None).await;
                    let _ = tx.send(AppMessage::CredStuffFinished(results));
                    ctx_clone.request_repaint();
                });
            }
            if self.cred_in_progress { ui.spinner(); ui.label(RichText::new("Testing credentials…").color(theme::WARN)); }
        });

        if !self.cred_results.is_empty() {
            ui.add_space(8.0);
            let successful: Vec<_> = self.cred_results.iter().filter(|r| r.success).collect();
            card(ui, format!("Results — {} / {} successful", successful.len(), self.cred_results.len()).as_str(), theme::OK, |ui| {
                ScrollArea::vertical().max_height(260.0).show(ui, |ui| {
                    if successful.is_empty() {
                        ui.label(RichText::new("No valid credentials found.").color(theme::TEXT_DIM));
                    }
                    for res in &successful {
                        egui::Frame::default().fill(theme::BG_DEEP).rounding(Rounding::same(6.0))
                            .inner_margin(Margin::symmetric(10.0, 6.0)).outer_margin(egui::Margin { bottom: 4.0, ..Default::default() })
                            .show(ui, |ui| {
                                ui.set_min_width(ui.available_width());
                                ui.label(RichText::new(format!("🔑  {}:{}", res.username, res.password)).color(theme::OK).monospace().size(12.5));
                            });
                    }
                });
            });
        }
    }

    // ── Spam & Flood ───────────────────────────────────────────
    fn tab_spam(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.label(RichText::new("Spam & Flood Testing").size(22.0).strong().color(theme::TEXT));
        ui.add_space(8.0);
        card(ui, "Rate-limit Tester", theme::WARN, |ui| {
            ui.horizontal(|ui| {
                field_label(ui, "MODULE");
                egui::ComboBox::from_id_source("spam_kind_combo").selected_text(&self.spam_kind).width(160.0).show_ui(ui, |ui| {
                    for k in &["db-flood", "comment-spam", "reg-spam"] {
                        ui.selectable_value(&mut self.spam_kind, k.to_string(), *k);
                    }
                });
                ui.add_space(12.0);
                field_label(ui, "ENDPOINT URL");
                ui.add(egui::TextEdit::singleline(&mut self.spam_endpoint).hint_text("https://example.com/api").desired_width(280.0));
            });
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                field_label(ui, "COUNT");
                ui.add(egui::DragValue::new(&mut self.spam_count).clamp_range(1..=100000));
                ui.add_space(14.0);
                field_label(ui, "THREADS");
                ui.add(egui::DragValue::new(&mut self.spam_threads).clamp_range(1..=100));
                ui.add_space(14.0);
                field_label(ui, "RATE (RPS)");
                ui.add(egui::DragValue::new(&mut self.spam_rate).clamp_range(1..=100));
                ui.add_space(14.0);
                field_label(ui, "PROXY");
                ui.add(egui::TextEdit::singleline(&mut self.spam_proxy).hint_text("optional").desired_width(180.0));
            });
        });
        ui.horizontal(|ui| {
            if ui.add(egui::Button::new(RichText::new("✉  SEND FLOOD").strong().color(Color32::BLACK)).fill(theme::WARN).rounding(Rounding::same(8.0)).min_size(egui::vec2(160.0, 34.0))).clicked()
                && !self.spam_in_progress && !self.spam_endpoint.is_empty() {
                self.spam_in_progress = true;
                self.spam_result = None;
                let endpoint = self.spam_endpoint.clone();
                let count = self.spam_count;
                let threads = self.spam_threads;
                let proxy = if self.spam_proxy.is_empty() { None } else { Some(self.spam_proxy.clone()) };
                let rate = self.spam_rate;
                let kind = self.spam_kind.clone();
                let limiter = create_rate_limiter(rate);
                let tx = self.tx.clone();
                let ctx_clone = ctx.clone();
                self.add_log(format!("{} flood started against {}", kind, endpoint));
                tokio::spawn(async move {
                    let sent = match kind.as_str() {
                        "comment-spam" => {
                            spam::comment_spam(&endpoint, "comment", "name", "email", "Great post! #__NUM__", count, threads, proxy.as_deref(), limiter).await
                        }
                        "reg-spam" => {
                            let fields = vec![("username", "__USERNAME__"), ("email", "__EMAIL__"), ("password", "__PASSWORD__")];
                            spam::registration_spam(&endpoint, &fields, count, threads, proxy.as_deref(), limiter).await
                        }
                        _ => {
                            let fields = vec![("data", "__RANDOM__"), ("timestamp", "__RANDOM_NUMBER__")];
                            flood_database(&endpoint, &fields, count, threads, proxy.as_deref(), limiter).await
                        }
                    };
                    let _ = tx.send(AppMessage::SpamFinished(sent));
                    ctx_clone.request_repaint();
                });
            }
            if self.spam_in_progress { ui.spinner(); ui.label(RichText::new("Sending…").color(theme::WARN)); }
        });
        if let Some(result) = self.spam_result {
            ui.add_space(8.0);
            egui::Frame::default().fill(theme::OK.gamma_multiply(0.1)).rounding(Rounding::same(8.0)).inner_margin(Margin::same(10.0)).show(ui, |ui| {
                ui.label(RichText::new(format!("✓ Completed — {} requests sent", result)).color(theme::OK).size(13.0));
            });
        }
    }

    // ── Payload Generator ──────────────────────────────────────
    fn tab_payload(&mut self, ui: &mut egui::Ui) {
        ui.label(RichText::new("Payload Generator").size(22.0).strong().color(theme::TEXT));
        ui.add_space(8.0);
        card(ui, "Generator Options", theme::PURPLE, |ui| {
            ui.horizontal(|ui| {
                field_label(ui, "TYPE");
                egui::ComboBox::from_id_source("payload_type_combo").selected_text(&self.payload_type).width(130.0).show_ui(ui, |ui| {
                    for t in &["reverse", "bind", "webshell", "downloadexec"] {
                        ui.selectable_value(&mut self.payload_type, t.to_string(), *t);
                    }
                });
                ui.add_space(12.0);
                field_label(ui, "PLATFORM");
                egui::ComboBox::from_id_source("payload_platform_combo").selected_text(&self.payload_platform).width(130.0).show_ui(ui, |ui| {
                    for p in &["linux", "windows", "macos", "python", "php", "nodejs", "ruby", "perl"] {
                        ui.selectable_value(&mut self.payload_platform, p.to_string(), *p);
                    }
                });
            });
            ui.add_space(8.0);
            if self.payload_type == "reverse" || self.payload_type == "bind" {
                ui.horizontal(|ui| {
                    field_label(ui, "LHOST");
                    ui.add(egui::TextEdit::singleline(&mut self.payload_lhost).hint_text("10.10.10.10").desired_width(160.0));
                    ui.add_space(12.0);
                    field_label(ui, "LPORT");
                    ui.add(egui::DragValue::new(&mut self.payload_lport).clamp_range(1..=65535));
                });
            }
            if self.payload_type == "downloadexec" {
                ui.horizontal(|ui| { field_label(ui, "PAYLOAD URL"); ui.add(egui::TextEdit::singleline(&mut self.payload_url).desired_width(300.0)); });
            }
            if self.payload_type == "webshell" {
                ui.horizontal(|ui| { field_label(ui, "PASSWORD (blank = random)"); ui.add(egui::TextEdit::singleline(&mut self.payload_password).desired_width(200.0)); });
            }
        });
        ui.horizontal(|ui| {
            if ui.add(egui::Button::new(RichText::new("❯  GENERATE").strong().color(Color32::BLACK)).fill(theme::PURPLE).rounding(Rounding::same(8.0)).min_size(egui::vec2(160.0, 34.0))).clicked() {
                let plat = match self.payload_platform.as_str() {
                    "windows" => Platform::Windows,
                    "macos" => Platform::MacOS,
                    "python" => Platform::Python,
                    "php" => Platform::PHP,
                    "nodejs" => Platform::NodeJS,
                    "ruby" => Platform::Ruby,
                    "perl" => Platform::Perl,
                    _ => Platform::Linux,
                };
                self.payload_generated = match self.payload_type.as_str() {
                    "bind" => generate_bind_shell(self.payload_lport, plat),
                    "webshell" => {
                        let pass = if self.payload_password.is_empty() { random_webshell_password() } else { self.payload_password.clone() };
                        generate_php_webshell(&pass)
                    }
                    "downloadexec" => generate_download_exec(&self.payload_url, plat),
                    _ => {
                        if self.payload_lhost.is_empty() {
                            "⚠ Reverse shell requires LHOST to be set.".to_string()
                        } else {
                            generate_reverse_shell(&self.payload_lhost, self.payload_lport, plat)
                        }
                    }
                };
                self.add_log(format!("Payload generated: {} / {}", self.payload_type, self.payload_platform));
            }
            if !self.payload_generated.is_empty() {
                if ui.button("💾 Save to file").clicked() {
                    if let Some(path) = rfd::FileDialog::new().save_file() {
                        let _ = std::fs::write(&path, &self.payload_generated);
                        self.add_log(format!("Payload saved to {}", path.display()));
                    }
                }
                if ui.button("📋 Copy").clicked() {
                    let text = self.payload_generated.clone();
                    ui.output_mut(|o| o.copied_text = text);
                }
            }
        });
        if !self.payload_generated.is_empty() {
            ui.add_space(8.0);
            code_block(ui, "payload", &self.payload_generated, 320.0);
        } else {
            ui.add_space(8.0);
            ui.label(RichText::new("Generated payload will appear here.").size(12.0).color(theme::TEXT_FAINT));
        }
    }

    // ── Report Viewer (rendered in-app) ────────────────────────
    fn tab_report(&mut self, ui: &mut egui::Ui) {
        ui.label(RichText::new("Report Viewer").size(22.0).strong().color(theme::TEXT));
        ui.label(RichText::new("Open a scan JSON file — the report is rendered right here, no browser needed.").size(12.5).color(theme::TEXT_DIM));
        ui.add_space(8.0);

        card(ui, "Load Scan Result", theme::OK, |ui| {
            ui.horizontal(|ui| {
                field_label(ui, "JSON FILE");
                ui.add(egui::TextEdit::singleline(&mut self.rv_json_path).hint_text("~/scan_results.json").desired_width(360.0));
                if ui.button("📂 Browse").clicked() {
                    if let Some(p) = rfd::FileDialog::new().add_filter("Scan JSON", &["json"]).pick_file() {
                        self.rv_json_path = p.display().to_string();
                        self.load_scan_json();
                    }
                }
                if ui.button("⤓ Load").clicked() { self.load_scan_json(); }
                if !self.rv_result.is_none() || !self.rv_json_path.is_empty() {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("🌐 Open HTML in browser").clicked() {
                            if let Some(res) = &self.rv_result {
                                let tmp = std::env::temp_dir().join("htool_report_preview.html");
                                let _ = save_html_report(res, tmp.to_string_lossy().as_ref());
                                let _ = open::that(tmp.to_string_lossy().to_string());
                            }
                        }
                    });
                }
            });
            if let Some(err) = &self.rv_error {
                ui.add_space(6.0);
                ui.label(RichText::new(format!("✗ {}", err)).color(theme::DANGER).size(12.0));
            }
        });

        if let Some(res) = self.rv_result.clone() {
            ui.horizontal(|ui| {
                if ui.button("▤  Rendered Report").clicked() { self.rv_show_json = false; }
                if ui.button("{}  JSON Data").clicked() { self.rv_show_json = true; }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("📄 Export HTML").clicked() {
                        if let Some(path) = rfd::FileDialog::new().add_filter("HTML Report", &["html"]).set_file_name("htool_report.html").save_file() {
                            let _ = save_html_report(&res, path.display().to_string().as_str());
                            self.add_log(format!("HTML report exported to {}", path.display()));
                        }
                    }
                });
            });
            ui.add_space(4.0);
            if self.rv_show_json {
                json_viewer(ui, &self.rv_json_text, ui.available_height() - 40.0);
            } else {
                ScrollArea::vertical().id_source("rv_scroll").show(ui, |ui| {
                    self.render_report_preview(ui, &res);
                });
            }
        }
    }

    // ── CVE Search ─────────────────────────────────────────────
    fn tab_cve(&mut self, ui: &mut egui::Ui) {
        ui.label(RichText::new("Offline CVE Database").size(22.0).strong().color(theme::TEXT));
        ui.add_space(8.0);
        card(ui, "Search", theme::DANGER, |ui| {
            ui.horizontal(|ui| {
                field_label(ui, "QUERY");
                let resp = ui.add(egui::TextEdit::singleline(&mut self.cve_query).hint_text("Apache, Log4j, Redis, WordPress…").desired_width(300.0));
                let search_clicked = ui.add(egui::Button::new(RichText::new("⌕  Search").color(Color32::BLACK)).fill(theme::ACCENT).rounding(Rounding::same(6.0))).clicked()
                    || (resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)));
                if search_clicked && !self.cve_query.is_empty() {
                    self.cve_results = search_cves(&self.cve_query);
                    self.add_log(format!("CVE search \"{}\" — {} matches", self.cve_query, self.cve_results.len()));
                }
                if ui.button("clear").clicked() { self.cve_results.clear(); self.cve_query.clear(); }
            });
        });

        if self.cve_results.is_empty() {
            ui.label(RichText::new("Type a keyword above, e.g. \"Apache\", \"nginx\", \"PHP\" — results come from the built-in offline database.").size(12.0).color(theme::TEXT_FAINT));
        } else {
            ui.label(RichText::new(format!("{} matching entries", self.cve_results.len())).size(13.0).color(theme::TEXT_DIM));
            ScrollArea::vertical().id_source("cve_scroll").show(ui, |ui| {
                for cve in &self.cve_results {
                    let sev_color = if cve.cvss_score >= 9.0 { theme::DANGER } else if cve.cvss_score >= 7.0 { theme::ORANGE } else { theme::WARN };
                    egui::Frame::default().fill(theme::BG_CARD).rounding(Rounding::same(10.0))
                        .stroke(Stroke::new(1.0_f32, theme::STROKE)).inner_margin(Margin::same(12.0))
                        .outer_margin(egui::Margin { bottom: 8.0, ..Default::default() })
                        .show(ui, |ui| {
                            ui.set_min_width(ui.available_width());
                            ui.horizontal(|ui| {
                                ui.label(RichText::new(&cve.id).strong().color(theme::ACCENT).monospace());
                                chip(ui, &format!("CVSS {:.1}", cve.cvss_score), sev_color, sev_color.gamma_multiply(0.12));
                                chip(ui, &cve.published_year.to_string(), theme::TEXT_DIM, theme::BG_ROW);
                            });
                            ui.label(RichText::new(format!("{} ({})", cve.product, cve.version_affected)).size(12.0).color(theme::CYAN));
                            ui.label(RichText::new(&cve.description).size(12.5).color(theme::TEXT_DIM));
                        });
                }
            });
        }
    }

    // ── Shared: full rendered report preview (mirrors the HTML export) ──
    fn render_report_preview(&self, ui: &mut egui::Ui, result: &ScanResult) {
        let sev = result.severity();
        let sev_c = severity_color(sev);

        // Header banner
        egui::Frame::default()
            .fill(theme::BG_CARD).rounding(Rounding::same(12.0))
            .stroke(Stroke::new(1.0_f32, theme::STROKE))
            .inner_margin(Margin::same(16.0))
            .outer_margin(egui::Margin { bottom: 12.0, ..Default::default() })
            .show(ui, |ui| {
                ui.set_min_width(ui.available_width());
                ui.horizontal(|ui| {
                    ui.label(RichText::new("◉ SCAN REPORT").size(19.0).strong().color(theme::ACCENT).monospace());
                    chip(ui, sev, sev_c, sev_c.gamma_multiply(0.13));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(RichText::new(&result.timestamp).size(11.5).color(theme::TEXT_FAINT).monospace());
                    });
                });
                ui.label(RichText::new(format!("target: {}", result.target)).size(13.0).color(theme::TEXT_DIM).monospace());
            });

        // Summary stat grid (5 per row)
        egui::Grid::new("stat_grid").min_col_width(118.0).spacing([8.0, 8.0]).show(ui, |ui| {
            stat_card(ui, result.open_ports.len(), "OPEN PORTS", theme::ACCENT);
            stat_card(ui, result.sql_vulnerable.len(), "SQL INJECTION", theme::DANGER);
            stat_card(ui, result.xss_vulnerable.len(), "XSS", theme::DANGER);
            stat_card(ui, result.subdomain_takeovers.len(), "TAKEOVERS", theme::DANGER);
            stat_card(ui, result.zone_transfers.len(), "ZONE TRANSFERS", theme::ORANGE);
            ui.end_row();
            stat_card(ui, result.cve_matches.len(), "CVE MATCHES", theme::WARN);
            stat_card(ui, result.discovered_paths.len(), "PATHS FOUND", theme::CYAN);
            stat_card(ui, result.subdomains.len(), "SUBDOMAINS", theme::CYAN);
            stat_card(ui, result.technologies.len(), "TECHNOLOGIES", theme::PURPLE);
            stat_card(ui, result.errors.len(), "ERRORS", theme::TEXT_DIM);
            ui.end_row();
        });
        ui.add_space(4.0);

        // Findings
        card(ui, &format!("🔓 Open Ports ({})", result.open_ports.len()), theme::ACCENT, |ui| {
            chip_grid(ui, &result.open_ports.iter().map(|(p, s)| format!("{} · {}", p, s)).collect::<Vec<_>>(), theme::CYAN);
        });
        card(ui, &format!("🐍 SQL Injection ({})", result.sql_vulnerable.len()), theme::DANGER, |ui| {
            vuln_list(ui, &result.sql_vulnerable, theme::DANGER);
        });
        card(ui, &format!("🕸 Cross-Site Scripting ({})", result.xss_vulnerable.len()), theme::DANGER, |ui| {
            vuln_list(ui, &result.xss_vulnerable, theme::ORANGE);
        });
        if !result.subdomain_takeovers.is_empty() {
            card(ui, &format!("⚠ Subdomain Takeovers ({})", result.subdomain_takeovers.len()), theme::DANGER, |ui| {
                vuln_list(ui, &result.subdomain_takeovers, theme::DANGER);
            });
        }
        if !result.zone_transfers.is_empty() {
            card(ui, &format!("⚠ DNS Zone Transfers ({})", result.zone_transfers.len()), theme::DANGER, |ui| {
                vuln_list(ui, &result.zone_transfers, theme::DANGER);
            });
        }
        if !result.cve_matches.is_empty() {
            card(ui, &format!("⚠ Known CVEs ({})", result.cve_matches.len()), theme::ORANGE, |ui| {
                vuln_list(ui, &result.cve_matches, theme::WARN);
            });
        }
        card(ui, &format!("📁 Discovered Paths ({})", result.discovered_paths.len()), theme::CYAN, |ui| {
            chip_grid(ui, &result.discovered_paths, theme::CYAN);
        });
        if !result.subdomains.is_empty() {
            card(ui, &format!("🌐 Subdomains ({})", result.subdomains.len()), theme::CYAN, |ui| {
                chip_grid(ui, &result.subdomains, theme::CYAN);
            });
        }
        if !result.technologies.is_empty() {
            card(ui, &format!("🛠 Detected Technologies ({})", result.technologies.len()), theme::PURPLE, |ui| {
                chip_grid(ui, &result.technologies, theme::CYAN);
            });
        }
        card(ui, "🔒 SSL / TLS", theme::ACCENT, |ui| {
            code_block(ui, "ssl", &result.ssl_info, 90.0);
        });
        card(ui, "🛡 Security Headers", theme::ACCENT, |ui| {
            code_block(ui, "headers", &result.security_headers, 140.0);
        });
        if !result.errors.is_empty() {
            card(ui, &format!("❌ Errors ({})", result.errors.len()), theme::TEXT_DIM, |ui| {
                vuln_list(ui, &result.errors, theme::TEXT_FAINT);
            });
        }
    }
}

// ══════════════════════════════════════════════════════════════════
//  Entry point
// ══════════════════════════════════════════════════════════════════

fn main() -> eframe::Result<()> {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let _guard = rt.enter();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 860.0])
            .with_min_inner_size([980.0, 640.0])
            .with_title("htool — Ultimate Hacker Toolkit")
            .with_icon(load_icon()),
        ..Default::default()
    };
    eframe::run_native(
        "htool — Ultimate Hacker Toolkit",
        options,
        Box::new(|_cc| Box::new(UltimateApp::default())),
    )
}

fn load_icon() -> egui::IconData {
    let bytes = include_bytes!("../assets/icon.png");
    let img = image::load_from_memory(bytes).expect("Failed to load icon");
    let rgba = img.to_rgba8();
    egui::IconData {
        width: rgba.width(),
        height: rgba.height(),
        rgba: rgba.into_raw(),
    }
}
