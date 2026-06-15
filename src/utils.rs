//! Shared utilities for the Ultimate Hacker Toolkit
//! Rate limiting, wordlist loading, HTTP client building,
//! proxy rotation, user-agent rotation, and common helpers.

use governor::{Quota, DefaultDirectRateLimiter};
use std::num::NonZeroU32;
use std::sync::Arc;
use tokio::sync::Mutex;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::time::Duration;

/// Shared rate limiter type used across all modules
pub type SharedRateLimiter = Arc<Mutex<DefaultDirectRateLimiter>>;

/// Create a new rate limiter with given requests per second
pub fn create_rate_limiter(rps: u32) -> SharedRateLimiter {
    let quota = Quota::per_second(NonZeroU32::new(rps).unwrap());
    Arc::new(Mutex::new(DefaultDirectRateLimiter::direct(quota)))
}

/// Wait for rate limiter approval
pub async fn throttle(limiter: &SharedRateLimiter) {
    let _ = limiter.lock().await.check().ok();
}

/// Load a wordlist from file (one entry per line). Returns default if file not found.
pub fn load_wordlist(path: Option<&str>) -> Vec<String> {
    if let Some(p) = path {
        std::fs::read_to_string(p)
            .unwrap_or_default()
            .lines()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    } else {
        // Default wordlist for directory brute, credentials, etc.
        vec![
            "admin".to_string(), "login".to_string(), "dashboard".to_string(),
            "wp-admin".to_string(), "backup".to_string(), ".env".to_string(),
            "config".to_string(), "api".to_string(), "v1".to_string(),
            "test".to_string(), "dev".to_string(), "staging".to_string(),
            "phpmyadmin".to_string(), "cgi-bin".to_string(), "robots.txt".to_string(),
            "sitemap.xml".to_string(), "swagger.json".to_string(), "openapi.json".to_string(),
            "graphql".to_string(), "health".to_string(), "status".to_string(),
            "metrics".to_string(), "dump.sql".to_string(), "backup.zip".to_string(),
            ".git/config".to_string(), ".git/HEAD".to_string(), "server-status".to_string(),
            "info.php".to_string(), "phpinfo.php".to_string(), "test.php".to_string(),
        ]
    }
}

/// Build an HTTP client with optional proxy, timeout, and user-agent
pub fn build_http_client(proxy: Option<&str>, timeout_secs: u64, user_agent: &str) -> reqwest::Client {
    let mut builder = reqwest::Client::builder()
        .timeout(Duration::from_secs(timeout_secs))
        .user_agent(user_agent);
    if let Some(proxy_url) = proxy {
        if let Ok(proxy) = reqwest::Proxy::all(proxy_url) {
            builder = builder.proxy(proxy);
        }
    }
    builder.build().expect("Failed to build HTTP client")
}

/// Common user agents for rotation
pub static USER_AGENTS: &[&str] = &[
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/115.0",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Safari/605.1.15",
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
    "Mozilla/5.0 (iPhone; CPU iPhone OS 17_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Mobile/15E148 Safari/604.1",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36 Edg/119.0.0.0",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:102.0) Gecko/20100101 Firefox/102.0",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
];

/// Get a random user agent string
pub fn random_user_agent() -> String {
    USER_AGENTS.choose(&mut thread_rng()).unwrap().to_string()
}

/// Parse proxy list from a file (one proxy per line, format: http://ip:port)
pub fn load_proxy_list(path: &str) -> Vec<String> {
    std::fs::read_to_string(path)
        .unwrap_or_default()
        .lines()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

/// Get a random proxy from a list
pub fn random_proxy(proxies: &[String]) -> Option<String> {
    proxies.choose(&mut thread_rng()).cloned()
}

/// Parse a target URL and extract host, port, protocol
pub fn parse_target(target: &str) -> (String, u16, String) {
    let normalized = if target.starts_with("http") {
        target.to_string()
    } else {
        format!("https://{}", target)
    };
    if let Ok(url) = url::Url::parse(&normalized) {
        let host = url.host_str().unwrap_or("localhost").to_string();
        let port = url.port().unwrap_or(if url.scheme() == "https" { 443 } else { 80 });
        let protocol = url.scheme().to_string();
        (host, port, protocol)
    } else {
        (target.to_string(), 80, "http".to_string())
    }
}

/// Generate a random string of given length
pub fn random_string(len: usize) -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut rng = rand::thread_rng();
    (0..len)
        .map(|_| CHARSET[rng.gen_range(0..CHARSET.len())] as char)
        .collect()
}

/// Check if a string looks like a valid IP address
pub fn is_ip_address(s: &str) -> bool {
    regex::Regex::new(r"^(\d{1,3}\.){3}\d{1,3}$").unwrap().is_match(s)
}

/// Extract domain from URL (remove subdomains)
pub fn extract_domain(host: &str) -> String {
    let parts: Vec<&str> = host.split('.').collect();
    if parts.len() >= 2 {
        format!("{}.{}", parts[parts.len()-2], parts[parts.len()-1])
    } else {
        host.to_string()
    }
}

/// Write data to a file, creating parent directories if needed
pub fn write_file(path: &str, contents: &str) -> Result<(), std::io::Error> {
    use std::fs;
    use std::path::Path;
    if let Some(parent) = Path::new(path).parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, contents)
}

/// Read file contents as string
pub fn read_file(path: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}

/// Convert a vector of bytes to hex string
pub fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

/// Check if a port is commonly used for web services
pub fn is_web_port(port: u16) -> bool {
    matches!(port, 80 | 443 | 8080 | 8443 | 8000 | 8888 | 3000 | 5000)
}

/// Get service name by port number
pub fn get_service_name(port: u16) -> &'static str {
    match port {
        21 => "FTP",
        22 => "SSH",
        23 => "Telnet",
        25 => "SMTP",
        53 => "DNS",
        80 => "HTTP",
        110 => "POP3",
        135 => "RPC",
        139 => "NetBIOS",
        143 => "IMAP",
        443 => "HTTPS",
        445 => "SMB",
        993 => "IMAPS",
        995 => "POP3S",
        1723 => "PPTP",
        3306 => "MySQL",
        3389 => "RDP",
        5900 => "VNC",
        8080 => "HTTP-Alt",
        8443 => "HTTPS-Alt",
        _ => "Unknown",
    }
}