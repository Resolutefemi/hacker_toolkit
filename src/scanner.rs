//! Core vulnerability scanner module
//! Port scanning, SQLi, XSS, directory brute, subdomain enum, SSL, security headers, CVE checks.

use crate::utils::{throttle, SharedRateLimiter, build_http_client, parse_target, get_service_name};
use crate::cve;
use reqwest::Client;
use url::Url;
use chrono::Local;
use serde::{Serialize, Deserialize};

/// Complete scan result structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    pub timestamp: String,
    pub target: String,
    pub open_ports: Vec<(u16, String)>,  // (port, service_name)
    pub sql_vulnerable: Vec<String>,
    pub xss_vulnerable: Vec<String>,
    pub discovered_paths: Vec<String>,
    pub subdomains: Vec<String>,
    pub ssl_info: String,
    pub security_headers: String,
    pub cve_matches: Vec<String>,
    pub errors: Vec<String>,
}

impl ScanResult {
    pub fn new(target: String) -> Self {
        Self {
            timestamp: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            target,
            open_ports: Vec::new(),
            sql_vulnerable: Vec::new(),
            xss_vulnerable: Vec::new(),
            discovered_paths: Vec::new(),
            subdomains: Vec::new(),
            ssl_info: String::new(),
            security_headers: String::new(),
            cve_matches: Vec::new(),
            errors: Vec::new(),
        }
    }
}

/// Scan configuration
pub struct ScannerConfig {
    pub scan_type: ScanType,
    pub rate_limit_rps: u32,
    pub proxy: Option<String>,
    pub wordlist: Vec<String>,
    pub timeout_secs: u64,
    pub user_agent: String,
}

pub enum ScanType {
    Quick,
    Full,
}

impl Default for ScannerConfig {
    fn default() -> Self {
        Self {
            scan_type: ScanType::Quick,
            rate_limit_rps: 10,
            proxy: None,
            wordlist: crate::utils::load_wordlist(None),
            timeout_secs: 8,
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36".to_string(),
        }
    }
}

// ========== Port Scanning ==========
pub async fn scan_ports(host: &str, ports: &[u16], limiter: &SharedRateLimiter) -> Vec<(u16, String)> {
    let mut open = Vec::new();
    for &port in ports {
        throttle(limiter).await;
        let addr = format!("{}:{}", host, port);
        if tokio::net::TcpStream::connect(&addr).await.is_ok() {
            let service = get_service_name(port);
            open.push((port, service.to_string()));
        }
    }
    open
}

// ========== Directory Bruteforce ==========
pub async fn dir_bruteforce(
    client: &Client,
    base_url: &Url,
    wordlist: &[String],
    limiter: &SharedRateLimiter,
) -> Vec<String> {
    let mut found = Vec::new();
    for path in wordlist {
        throttle(limiter).await;
        let url = match base_url.join(path) {
            Ok(u) => u,
            Err(_) => continue,
        };
        match client.get(url.clone()).send().await {
            Ok(resp) => {
                if resp.status().is_success() {
                    found.push(url.to_string());
                }
            }
            Err(_) => continue,
        }
    }
    found
}

// ========== SQL Injection ==========
pub async fn sql_injection_test(
    client: &Client,
    base_url: &Url,
    limiter: &SharedRateLimiter,
) -> Vec<String> {
    let mut vuln = Vec::new();
    let payloads = vec![
        "'",
        "1' OR '1'='1",
        "1' UNION SELECT NULL--",
        "\" OR \"1\"=\"1",
        "'; DROP TABLE users; --",
        "1 AND 1=1",
        "1 AND 1=2",
        "' OR 1=1-- -",
        "admin' --",
        "1' AND SLEEP(5)-- -",
        "1' WAITFOR DELAY '0:0:5'-- -",
    ];
    let params = vec!["id", "page", "cat", "product", "user", "post", "article", "news", "q", "search"];

    for param in params {
        for payload in &payloads {
            throttle(limiter).await;
            let mut url = base_url.clone();
            url.query_pairs_mut().append_pair(param, payload);
            match client.get(url.clone()).send().await {
                Ok(resp) => {
                    let body = resp.text().await.unwrap_or_default().to_lowercase();
                    if body.contains("sql") || body.contains("mysql") || body.contains("syntax")
                        || body.contains("unclosed") || body.contains("odbc")
                        || body.contains("oracle") || body.contains("postgresql") {
                        vuln.push(url.to_string());
                        break;
                    }
                }
                Err(_) => continue,
            }
        }
    }
    vuln
}

// ========== XSS ==========
pub async fn xss_test(client: &Client, base_url: &Url, limiter: &SharedRateLimiter) -> Vec<String> {
    let mut vuln = Vec::new();
    let payloads = vec![
        "<script>alert(1)</script>",
        "<img src=x onerror=alert(1)>",
        "\"><script>alert(1)</script>",
        "javascript:alert(1)",
        "<body onload=alert(1)>",
        "<svg onload=alert(1)>",
        "<a href=\"javascript:alert(1)\">click</a>",
        "<iframe src=\"javascript:alert(1)\">",
        "<input onfocus=alert(1) autofocus>",
        "';alert(1);//",
        "-alert(1)-",
        "<script>confirm(1)</script>",
    ];
    let params = vec!["search", "q", "s", "keyword", "name", "query", "term", "text", "comment"];

    for param in params {
        for payload in &payloads {
            throttle(limiter).await;
            let mut url = base_url.clone();
            url.query_pairs_mut().append_pair(param, payload);
            match client.get(url.clone()).send().await {
                Ok(resp) => {
                    let body = resp.text().await.unwrap_or_default().to_lowercase();
                    if body.contains("<script") || body.contains("onerror")
                        || body.contains("alert") || body.contains("javascript:")
                        || body.contains("confirm") {
                        vuln.push(url.to_string());
                        break;
                    }
                }
                Err(_) => continue,
            }
        }
    }
    vuln
}

// ========== Subdomain Enumeration ==========
pub async fn subdomain_enum(domain: &str, limiter: &SharedRateLimiter) -> Vec<String> {
    let prefixes = vec![
        "www", "mail", "ftp", "admin", "dev", "test", "api", "vpn", "blog", "shop",
        "webmail", "cpanel", "whm", "ns1", "ns2", "secure", "portal", "dashboard",
        "stage", "staging", "backup", "cloud", "cdn", "files", "static", "img",
        "assets", "media", "video", "stream", "chat", "support", "help", "docs",
        "wiki", "news", "forum", "community", "store", "shop", "buy", "cart",
    ];
    let mut found = Vec::new();
    for prefix in prefixes {
        throttle(limiter).await;
        let sub = format!("{}.{}", prefix, domain);
        let addr = format!("{}:80", sub);
        if tokio::net::TcpStream::connect(&addr).await.is_ok() {
            found.push(sub);
        }
    }
    found
}

// ========== SSL/TLS Check ==========
pub async fn check_ssl(host: &str) -> String {
    match native_tls::TlsConnector::builder()
        .danger_accept_invalid_certs(true)
        .build()
    {
        Ok(connector) => match std::net::TcpStream::connect(format!("{}:443", host)) {
            Ok(stream) => match connector.connect(host, stream) {
                Ok(_) => "✅ SSL/TLS handshake successful (certificate may be self‑signed)".to_string(),
                Err(e) => format!("❌ SSL handshake failed: {}", e),
            },
            Err(e) => format!("❌ Cannot connect to port 443: {}", e),
        },
        Err(e) => format!("❌ TLS connector error: {}", e),
    }
}

// ========== Security Headers ==========
pub async fn get_security_headers(client: &Client, base_url: &Url) -> String {
    match client.get(base_url.clone()).send().await {
        Ok(resp) => {
            let headers = resp.headers();
            let important = vec![
                "Strict-Transport-Security",
                "Content-Security-Policy",
                "X-Frame-Options",
                "X-Content-Type-Options",
                "Referrer-Policy",
                "Permissions-Policy",
                "X-XSS-Protection",
            ];
            let mut found = Vec::new();
            for &h in &important {
                if let Some(val) = headers.get(h) {
                    found.push(format!("{}: {:?}", h, val));
                }
            }
            if found.is_empty() {
                "⚠️ No important security headers found.".to_string()
            } else {
                found.join("\n")
            }
        }
        Err(e) => format!("❌ Failed to fetch headers: {}", e),
    }
}

// ========== Main Orchestrator ==========
pub async fn run_full_scan(
    target: String,
    config: ScannerConfig,
    progress_callback: Option<Box<dyn Fn(f32) + Send>>,
) -> ScanResult {
    let mut result = ScanResult::new(target.clone());

    // Normalize target
    let (host, default_port, protocol) = parse_target(&target);
    let base_url_str = format!("{}://{}:{}", protocol, host, default_port);
    let base_url = match Url::parse(&base_url_str) {
        Ok(u) => u,
        Err(e) => {
            result.errors.push(format!("Invalid URL: {}", e));
            return result;
        }
    };

    // Build client
    let client = build_http_client(
        config.proxy.as_deref(),
        config.timeout_secs,
        &config.user_agent,
    );
    let limiter = crate::utils::create_rate_limiter(config.rate_limit_rps);

    // Port list
    let ports: Vec<u16> = match config.scan_type {
        ScanType::Quick => vec![21,22,23,25,53,80,110,135,139,143,443,445,993,995,1723,3306,3389,5900,8080,8443],
        ScanType::Full => (1..=1024).collect(),
    };

    // 1. Port scan
    result.open_ports = scan_ports(&host, &ports, &limiter).await;
    if let Some(cb) = &progress_callback { cb(0.2); }

    // 2. Directory brute
    result.discovered_paths = dir_bruteforce(&client, &base_url, &config.wordlist, &limiter).await;
    if let Some(cb) = &progress_callback { cb(0.4); }

    // 3. SQLi
    result.sql_vulnerable = sql_injection_test(&client, &base_url, &limiter).await;
    if let Some(cb) = &progress_callback { cb(0.6); }

    // 4. XSS
    result.xss_vulnerable = xss_test(&client, &base_url, &limiter).await;
    if let Some(cb) = &progress_callback { cb(0.7); }

    // 5. Subdomains (full only)
    if let ScanType::Full = config.scan_type {
        result.subdomains = subdomain_enum(&host, &limiter).await;
    }
    if let Some(cb) = &progress_callback { cb(0.8); }

    // 6. SSL
    result.ssl_info = check_ssl(&host).await;
    // 7. Security headers
    result.security_headers = get_security_headers(&client, &base_url).await;
    // 8. CVE matches
    result.cve_matches = cve::check_cves(&client, &base_url).await;

    if let Some(cb) = &progress_callback { cb(1.0); }
    result
}