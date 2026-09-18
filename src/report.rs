//! Report generation module
//! Creates modern HTML reports and JSON exports from scan results.

use crate::scanner::ScanResult;
use crate::credential::LoginResult;
use serde_json;
use std::fs;
use chrono::Local;

/// Severity color mapping for the HTML report
fn severity_palette(sev: &str) -> (&'static str, &'static str) {
    match sev {
        "CRITICAL" => ("#f87171", "rgba(248,113,113,0.12)"),
        "HIGH" => ("#fb923c", "rgba(251,146,60,0.12)"),
        "MEDIUM" => ("#fbbf24", "rgba(251,191,36,0.12)"),
        "LOW" => ("#38bdf8", "rgba(56,189,248,0.12)"),
        _ => ("#34d399", "rgba(52,211,153,0.12)"),
    }
}

fn chip(item: &str) -> String {
    let (fg, bg) = if item.starts_with("[WAF]") {
        ("#fb923c", "rgba(251,146,60,0.14)")
    } else {
        ("#38bdf8", "rgba(56,189,248,0.10)")
    };
    format!(r#"<span class="chip" style="color:{};background:{};">{}</span>"#, fg, bg, html_escape(item))
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

/// Generate an HTML report from a single scan result
pub fn generate_html_report(result: &ScanResult) -> String {
    let sev = result.severity();
    let (sev_fg, sev_bg) = severity_palette(sev);

    let mut html = String::new();

    html.push_str(&format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>htool — Scan Report · {target}</title>
<style>
    * {{ margin: 0; padding: 0; box-sizing: border-box; }}
    body {{
        font-family: 'Segoe UI', 'Inter', Tahoma, Geneva, Verdana, sans-serif;
        background: #070b14;
        background-image:
            radial-gradient(ellipse 900px 480px at 85% -10%, rgba(0,230,158,0.07), transparent),
            radial-gradient(ellipse 700px 420px at 8% 8%, rgba(56,189,248,0.06), transparent);
        color: #e2e8f0;
        padding: 2.5rem 1.25rem;
        line-height: 1.65;
    }}
    .container {{ max-width: 1080px; margin: 0 auto; }}
    .report-head {{
        background: #111729;
        border: 1px solid #2c3858;
        border-radius: 16px;
        padding: 1.75rem 2rem;
        margin-bottom: 1.25rem;
    }}
    .report-head h1 {{
        font-size: 1.9rem;
        color: #00e69e;
        letter-spacing: 0.5px;
        display: flex; align-items: center; gap: 0.75rem; flex-wrap: wrap;
    }}
    .sev-badge {{
        font-size: 0.78rem; font-weight: 700; letter-spacing: 1.5px;
        padding: 5px 14px; border-radius: 100px;
        color: {sev_fg}; background: {sev_bg};
        border: 1px solid {sev_fg};
    }}
    .target-row {{
        font-family: 'Cascadia Code', 'Fira Code', Consolas, monospace;
        color: #8c98b2; font-size: 0.95rem; margin-top: 0.5rem;
    }}
    .meta-row {{ color: #606a82; font-size: 0.82rem; margin-top: 0.35rem; }}
    .stat-grid {{
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
        gap: 10px; margin-bottom: 1.25rem;
    }}
    .stat {{
        background: #111729; border: 1px solid #2c3858;
        border-radius: 12px; padding: 0.9rem 1rem; text-align: center;
        transition: transform .15s ease, border-color .15s ease;
    }}
    .stat:hover {{ transform: translateY(-2px); border-color: #3d4c78; }}
    .stat .num {{ font-size: 1.55rem; font-weight: 700; }}
    .stat .lbl {{ font-size: 0.68rem; letter-spacing: 1.2px; color: #8c98b2; margin-top: 2px; }}
    .card {{
        background: #111729;
        border: 1px solid #2c3858;
        border-radius: 14px;
        padding: 1.4rem 1.6rem;
        margin-bottom: 1.1rem;
    }}
    .card h2 {{
        color: #e2e8f0;
        font-size: 1.08rem;
        margin-bottom: 0.9rem;
        display: flex; align-items: center; gap: 0.6rem;
    }}
    .card h2 .count {{ color: #8c98b2; font-weight: 400; }}
    .card.accent-green {{ border-left: 3px solid #00e69e; }}
    .card.accent-red   {{ border-left: 3px solid #f87171; }}
    .card.accent-orange{{ border-left: 3px solid #fb923c; }}
    .card.accent-cyan  {{ border-left: 3px solid #38bdf8; }}
    .card.accent-purple{{ border-left: 3px solid #a78bfa; }}
    .chips {{ display: flex; flex-wrap: wrap; gap: 8px; }}
    .chip {{
        padding: 4px 12px; border-radius: 100px;
        font-size: 0.8rem; font-family: 'Cascadia Code', 'Fira Code', Consolas, monospace;
    }}
    .vuln {{
        background: #050810; border: 1px solid rgba(248,113,113,0.25);
        border-radius: 8px; padding: 9px 14px; margin-bottom: 6px;
        font-family: 'Cascadia Code', 'Fira Code', Consolas, monospace;
        font-size: 0.82rem; word-break: break-all;
    }}
    .vuln.orange {{ border-color: rgba(251,146,60,0.3); }}
    .vuln.warn   {{ border-color: rgba(251,191,36,0.3); }}
    pre {{
        background: #050810;
        border: 1px solid #2c3858;
        padding: 1rem;
        border-radius: 10px;
        overflow-x: auto;
        font-family: 'Cascadia Code', 'Fira Code', Consolas, monospace;
        font-size: 0.8rem;
        color: #8c98b2;
        white-space: pre-wrap; word-break: break-word;
    }}
    .ok-pill {{
        display: inline-block; padding: 5px 14px; border-radius: 100px;
        background: rgba(52,211,153,0.12); color: #34d399;
        font-size: 0.8rem; font-weight: 600;
    }}
    .footer {{
        text-align: center; margin-top: 2rem; padding-top: 1.25rem;
        border-top: 1px solid #1c2440; font-size: 0.8rem; color: #606a82;
    }}
    .footer b {{ color: #00e69e; }}
    @media (max-width: 768px) {{
        body {{ padding: 1.25rem 0.75rem; }}
        .report-head {{ padding: 1.25rem 1.25rem; }}
        .report-head h1 {{ font-size: 1.45rem; }}
    }}
</style>
</head>
<body>
<div class="container">

<div class="report-head">
    <h1>◉ SCAN REPORT <span class="sev-badge">{sev} SEVERITY</span></h1>
    <div class="target-row">target&nbsp;&nbsp;→&nbsp;&nbsp;{target}</div>
    <div class="meta-row">generated {ts} · htool v{version} · authorised security assessment</div>
</div>

<div class="stat-grid">
    <div class="stat"><div class="num" style="color:#00e69e;">{ports}</div><div class="lbl">OPEN PORTS</div></div>
    <div class="stat"><div class="num" style="color:#f87171;">{sqli}</div><div class="lbl">SQL INJECTION</div></div>
    <div class="stat"><div class="num" style="color:#f87171;">{xss}</div><div class="lbl">XSS</div></div>
    <div class="stat"><div class="num" style="color:#f87171;">{takeovers}</div><div class="lbl">TAKEOVERS</div></div>
    <div class="stat"><div class="num" style="color:#fb923c;">{zt}</div><div class="lbl">ZONE TRANSFERS</div></div>
    <div class="stat"><div class="num" style="color:#fbbf24;">{cves}</div><div class="lbl">CVE MATCHES</div></div>
    <div class="stat"><div class="num" style="color:#38bdf8;">{paths}</div><div class="lbl">PATHS FOUND</div></div>
    <div class="stat"><div class="num" style="color:#38bdf8;">{subs}</div><div class="lbl">SUBDOMAINS</div></div>
    <div class="stat"><div class="num" style="color:#a78bfa;">{techs}</div><div class="lbl">TECHNOLOGIES</div></div>
</div>
"#,
        target = html_escape(&result.target),
        ts = html_escape(&result.timestamp),
        version = env!("CARGO_PKG_VERSION"),
        sev = sev,
        sev_fg = sev_fg,
        sev_bg = sev_bg,
        ports = result.open_ports.len(),
        sqli = result.sql_vulnerable.len(),
        xss = result.xss_vulnerable.len(),
        takeovers = result.subdomain_takeovers.len(),
        zt = result.zone_transfers.len(),
        cves = result.cve_matches.len(),
        paths = result.discovered_paths.len(),
        subs = result.subdomains.len(),
        techs = result.technologies.len(),
    ));

    // Open Ports
    html.push_str(&format!(
        r#"<div class="card accent-green"><h2>🔓 Open Ports <span class="count">({})</span></h2><div class="chips">{}</div></div>"#,
        result.open_ports.len(),
        if result.open_ports.is_empty() {
            r#"<span class="ok-pill">✓ no ports detected</span>"#.to_string()
        } else {
            result.open_ports.iter().map(|(p, s)| chip(&format!("{} · {}", p, s))).collect::<String>()
        }
    ));

    // SQL Injection
    html.push_str(&format!(
        r#"<div class="card accent-red"><h2>🐍 SQL Injection <span class="count">({})</span></h2>{}</div>"#,
        result.sql_vulnerable.len(),
        if result.sql_vulnerable.is_empty() {
            r#"<span class="ok-pill">✓ none found</span>"#.to_string()
        } else {
            result.sql_vulnerable.iter().map(|u| format!(r#"<div class="vuln">⚠ {}</div>"#, html_escape(u))).collect::<String>()
        }
    ));

    // XSS
    html.push_str(&format!(
        r#"<div class="card accent-red"><h2>🕸 Cross-Site Scripting (XSS) <span class="count">({})</span></h2>{}</div>"#,
        result.xss_vulnerable.len(),
        if result.xss_vulnerable.is_empty() {
            r#"<span class="ok-pill">✓ none found</span>"#.to_string()
        } else {
            result.xss_vulnerable.iter().map(|u| format!(r#"<div class="vuln orange">⚠ {}</div>"#, html_escape(u))).collect::<String>()
        }
    ));

    // Subdomain takeovers
    if !result.subdomain_takeovers.is_empty() {
        html.push_str(&format!(
            r#"<div class="card accent-red"><h2>⚠ Subdomain Takeovers <span class="count">({})</span></h2>{}</div>"#,
            result.subdomain_takeovers.len(),
            result.subdomain_takeovers.iter().map(|t| format!(r#"<div class="vuln">⚠ {}</div>"#, html_escape(t))).collect::<String>()
        ));
    }

    // Zone transfers
    if !result.zone_transfers.is_empty() {
        html.push_str(&format!(
            r#"<div class="card accent-red"><h2>⚠ DNS Zone Transfers <span class="count">({})</span></h2>{}</div>"#,
            result.zone_transfers.len(),
            result.zone_transfers.iter().map(|t| format!(r#"<div class="vuln">⚠ {}</div>"#, html_escape(t))).collect::<String>()
        ));
    }

    // CVE matches
    if !result.cve_matches.is_empty() {
        html.push_str(&format!(
            r#"<div class="card accent-orange"><h2>⚠ Known CVE Matches <span class="count">({})</span></h2>{}</div>"#,
            result.cve_matches.len(),
            result.cve_matches.iter().map(|c| format!(r#"<div class="vuln warn">⚠ {}</div>"#, html_escape(c))).collect::<String>()
        ));
    }

    // Discovered paths
    html.push_str(&format!(
        r#"<div class="card accent-cyan"><h2>📁 Discovered Paths <span class="count">({})</span></h2><div class="chips">{}</div></div>"#,
        result.discovered_paths.len(),
        if result.discovered_paths.is_empty() {
            r#"<span class="ok-pill">✓ no exposed paths</span>"#.to_string()
        } else {
            result.discovered_paths.iter().map(|p| chip(p)).collect::<String>()
        }
    ));

    // Subdomains
    if !result.subdomains.is_empty() {
        html.push_str(&format!(
            r#"<div class="card accent-cyan"><h2>🌐 Subdomains <span class="count">({})</span></h2><div class="chips">{}</div></div>"#,
            result.subdomains.len(),
            result.subdomains.iter().map(|s| chip(s)).collect::<String>()
        ));
    }

    // Technologies
    if !result.technologies.is_empty() {
        html.push_str(&format!(
            r#"<div class="card accent-purple"><h2>🛠 Detected Technologies <span class="count">({})</span></h2><div class="chips">{}</div></div>"#,
            result.technologies.len(),
            result.technologies.iter().map(|t| chip(t)).collect::<String>()
        ));
    }

    // SSL
    html.push_str(&format!(
        r#"<div class="card accent-green"><h2>🔒 SSL / TLS</h2><pre>{}</pre></div>"#,
        html_escape(&result.ssl_info)
    ));

    // Security headers
    html.push_str(&format!(
        r#"<div class="card accent-green"><h2>🛡 Security Headers</h2><pre>{}</pre></div>"#,
        html_escape(&result.security_headers)
    ));

    // Errors
    if !result.errors.is_empty() {
        html.push_str(&format!(
            r#"<div class="card"><h2>❌ Scan Errors <span class="count">({})</span></h2>{}</div>"#,
            result.errors.len(),
            result.errors.iter().map(|e| format!(r#"<div class="vuln" style="border-color:rgba(140,152,178,0.25);">{}</div>"#, html_escape(e))).collect::<String>()
        ));
    }

    html.push_str(&format!(
        r#"<div class="footer">
            Generated by <b>htool v{}</b> — Ultimate Hacker Toolkit · by Resolute Femi<br>
            For authorised security testing and educational purposes only
        </div>
    </div>
    </body>
    </html>"#,
        env!("CARGO_PKG_VERSION")
    ));

    html
}

/// Generate a JSON report from a scan result
pub fn generate_json_report(result: &ScanResult) -> String {
    serde_json::to_string_pretty(result).unwrap_or_else(|_| "{}".to_string())
}

/// Save HTML report to a file
pub fn save_html_report(result: &ScanResult, path: &str) -> Result<(), String> {
    let html = generate_html_report(result);
    fs::write(path, html).map_err(|e| format!("Failed to write HTML report: {}", e))
}

/// Save JSON report to a file
pub fn save_json_report(result: &ScanResult, path: &str) -> Result<(), String> {
    let json = generate_json_report(result);
    fs::write(path, json).map_err(|e| format!("Failed to write JSON report: {}", e))
}

/// Generate a combined report (HTML + JSON) with timestamped filenames
pub fn generate_combined_report(result: &ScanResult, base_name: &str) -> Result<(), String> {
    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let html_path = format!("{}_{}.html", base_name, timestamp);
    let json_path = format!("{}_{}.json", base_name, timestamp);
    save_html_report(result, &html_path)?;
    save_json_report(result, &json_path)?;
    Ok(())
}

/// Generate a report for credential stuffing results
pub fn generate_cred_report(results: &[LoginResult], output_path: &str) -> Result<(), String> {
    let successful: Vec<&LoginResult> = results.iter().filter(|r| r.success).collect();
    let mut content = String::new();
    content.push_str("Credential Stuffing Report\n");
    content.push_str(&format!("Generated: {}\n", Local::now().format("%Y-%m-%d %H:%M:%S")));
    content.push_str(&format!("Total attempts: {}\n", results.len()));
    content.push_str(&format!("Successful: {}\n", successful.len()));
    content.push_str("\nSuccessful credentials:\n");
    content.push_str("-".repeat(50).as_str());
    content.push_str("\n");
    for res in successful {
        content.push_str(&format!("{}:{}\n", res.username, res.password));
    }
    fs::write(output_path, content).map_err(|e| format!("Failed to write credential report: {}", e))
}
