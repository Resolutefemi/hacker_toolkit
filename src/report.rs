//! Report generation module
//! Creates beautiful HTML reports and JSON exports from scan results.

use crate::scanner::ScanResult;
use crate::credential::LoginResult;
use serde_json;
use std::fs;
use std::path::Path;
use chrono::Local;

/// Generate an HTML report from a single scan result
pub fn generate_html_report(result: &ScanResult) -> String {
    let mut html = String::new();
    
    html.push_str(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Vulnerability Scan Report</title>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body {
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            background: linear-gradient(135deg, #0a0e27 0%, #1a1f3a 100%);
            color: #e0e0e0;
            padding: 2rem;
            line-height: 1.6;
        }
        .container { max-width: 1200px; margin: 0 auto; }
        h1 {
            font-size: 2.5rem;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
            margin-bottom: 0.5rem;
        }
        .timestamp { color: #888; margin-bottom: 2rem; border-bottom: 1px solid #333; padding-bottom: 0.5rem; }
        .card {
            background: rgba(30, 30, 46, 0.8);
            border-radius: 12px;
            padding: 1.5rem;
            margin-bottom: 1.5rem;
            border-left: 4px solid #667eea;
            backdrop-filter: blur(5px);
        }
        .card h2 {
            color: #667eea;
            margin-bottom: 1rem;
            font-size: 1.5rem;
        }
        .badge {
            display: inline-block;
            padding: 4px 12px;
            border-radius: 20px;
            font-size: 0.8rem;
            font-weight: bold;
            margin-right: 8px;
            margin-bottom: 8px;
        }
        .badge-success { background: #10b981; color: white; }
        .badge-danger { background: #ef4444; color: white; }
        .badge-warning { background: #f59e0b; color: white; }
        .badge-info { background: #3b82f6; color: white; }
        .port-list { display: flex; flex-wrap: wrap; gap: 10px; margin-top: 10px; }
        .port-item {
            background: #1e1e2e;
            border: 1px solid #667eea;
            border-radius: 8px;
            padding: 5px 12px;
            font-family: monospace;
        }
        .vuln-url {
            background: #2d2d3a;
            padding: 8px;
            border-radius: 6px;
            margin: 5px 0;
            font-family: monospace;
            font-size: 0.85rem;
            word-break: break-all;
        }
        pre {
            background: #0d0d1a;
            padding: 1rem;
            border-radius: 8px;
            overflow-x: auto;
            font-family: monospace;
            font-size: 0.8rem;
        }
        .footer {
            text-align: center;
            margin-top: 2rem;
            padding-top: 1rem;
            border-top: 1px solid #333;
            font-size: 0.8rem;
            color: #666;
        }
        @media (max-width: 768px) {
            body { padding: 1rem; }
            h1 { font-size: 1.8rem; }
        }
    </style>
</head>
<body>
<div class="container">
"#);

    html.push_str(&format!(r#"
    <h1>🔍 Vulnerability Scan Report</h1>
    <div class="timestamp">
        <strong>Target:</strong> {} <br>
        <strong>Generated:</strong> {}
    </div>
"#, result.target, result.timestamp));

    // Open Ports
    html.push_str(&format!(r#"
    <div class="card">
        <h2>🔓 Open Ports ({})</h2>
        <div class="port-list">
"#, result.open_ports.len()));
    for (port, service) in &result.open_ports {
        html.push_str(&format!(r#"<div class="port-item">{} ({})</div>"#, port, service));
    }
    html.push_str(r#"
        </div>
    </div>
"#);

    // SQL Injection
    html.push_str(&format!(r#"
    <div class="card">
        <h2>🐍 SQL Injection ({})</h2>
"#, result.sql_vulnerable.len()));
    if result.sql_vulnerable.is_empty() {
        html.push_str(r#"<p class="badge badge-success">No obvious SQL injection found.</p>"#);
    } else {
        for url in &result.sql_vulnerable {
            html.push_str(&format!(r#"<div class="vuln-url">⚠️ {}</div>"#, url));
        }
    }
    html.push_str(r#"</div>"#);

    // XSS
    html.push_str(&format!(r#"
    <div class="card">
        <h2>🕸️ Cross-Site Scripting (XSS) ({})</h2>
"#, result.xss_vulnerable.len()));
    if result.xss_vulnerable.is_empty() {
        html.push_str(r#"<p class="badge badge-success">No obvious XSS found.</p>"#);
    } else {
        for url in &result.xss_vulnerable {
            html.push_str(&format!(r#"<div class="vuln-url">⚠️ {}</div>"#, url));
        }
    }
    html.push_str(r#"</div>"#);

    // Discovered Paths
    html.push_str(&format!(r#"
    <div class="card">
        <h2>📁 Discovered Paths ({})</h2>
        <div class="port-list">
"#, result.discovered_paths.len()));
    for path in &result.discovered_paths {
        html.push_str(&format!(r#"<div class="port-item">{}</div>"#, path));
    }
    html.push_str(r#"
        </div>
    </div>
"#);

    // Subdomains
    if !result.subdomains.is_empty() {
        html.push_str(&format!(r#"
        <div class="card">
            <h2>🌐 Subdomains ({})</h2>
            <div class="port-list">
        "#, result.subdomains.len()));
        for sub in &result.subdomains {
            html.push_str(&format!(r#"<div class="port-item">{}</div>"#, sub));
        }
        html.push_str(r#"
            </div>
        </div>
        "#);
    }

    // SSL Info
    html.push_str(&format!(r#"
    <div class="card">
        <h2>🔒 SSL/TLS</h2>
        <pre>{}</pre>
    </div>
"#, result.ssl_info));

    // Security Headers
    html.push_str(&format!(r#"
    <div class="card">
        <h2>🛡️ Security Headers</h2>
        <pre>{}</pre>
    </div>
"#, result.security_headers));

    // Technologies Fingerprinted
    if !result.technologies.is_empty() {
        html.push_str(&format!(r#"
        <div class="card">
            <h2>🛠️ Detected Technologies ({})</h2>
            <div class="port-list">
        "#, result.technologies.len()));
        for tech in &result.technologies {
            html.push_str(&format!(r#"<div class="port-item">{}</div>"#, tech));
        }
        html.push_str(r#"
            </div>
        </div>
        "#);
    }

    // Subdomain Takeovers
    if !result.subdomain_takeovers.is_empty() {
        html.push_str(&format!(r#"
        <div class="card" style="border-left: 4px solid #ef4444;">
            <h2>⚠️ Potential Subdomain Takeovers ({})</h2>
            <div class="port-list" style="flex-direction: column;">
        "#, result.subdomain_takeovers.len()));
        for takeover in &result.subdomain_takeovers {
            html.push_str(&format!(r#"<div class="vuln-url" style="color:#f87171;">{}</div>"#, takeover));
        }
        html.push_str(r#"
            </div>
        </div>
        "#);
    }

    // DNS Zone Transfers
    if !result.zone_transfers.is_empty() {
        html.push_str(&format!(r#"
        <div class="card" style="border-left: 4px solid #ef4444;">
            <h2>⚠️ Vulnerable DNS Zone Transfers ({})</h2>
            <div class="port-list" style="flex-direction: column;">
        "#, result.zone_transfers.len()));
        for zt in &result.zone_transfers {
            html.push_str(&format!(r#"<div class="vuln-url" style="color:#f87171;">{}</div>"#, zt));
        }
        html.push_str(r#"
            </div>
        </div>
        "#);
    }

    // CVE Matches
    if !result.cve_matches.is_empty() {
        html.push_str(&format!(r#"
        <div class="card">
            <h2>⚠️ Known CVEs ({})</h2>
            <div class="port-list">
        "#, result.cve_matches.len()));
        for cve in &result.cve_matches {
            html.push_str(&format!(r#"<div class="vuln-url">{}</div>"#, cve));
        }
        html.push_str(r#"
            </div>
        </div>
        "#);
    }

    // Errors
    if !result.errors.is_empty() {
        html.push_str(&format!(r#"
        <div class="card">
            <h2>❌ Errors</h2>
            <div class="port-list">
        "#));
        for err in &result.errors {
            html.push_str(&format!(r#"<div class="vuln-url" style="color:#f87171;">{}</div>"#, err));
        }
        html.push_str(r#"
            </div>
        </div>
        "#);
    }

    html.push_str(r#"
    <div class="footer">
        Generated by Ultimate Hacker Toolkit | For authorised security testing only
    </div>
</div>
</body>
</html>
"#);

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
    content.push_str(&format!("Credential Stuffing Report\n"));
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