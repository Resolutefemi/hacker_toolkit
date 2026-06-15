//! Shared utilities: rate limiting, wordlist loading, HTTP client builder

use governor::{RateLimiter, Quota};
use governor::clock::DefaultClock;
use std::num::NonZeroU32;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Shared rate limiter type used across all modules
pub type SharedRateLimiter = Arc<Mutex<RateLimiter<DefaultClock>>>;

/// Create a new rate limiter with given requests per second
pub fn create_rate_limiter(rps: u32) -> SharedRateLimiter {
    let quota = Quota::per_second(NonZeroU32::new(rps).unwrap());
    Arc::new(Mutex::new(RateLimiter::direct(quota)))
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
        // Default small wordlist for directory brute and credentials
        vec![
            "admin".to_string(), "login".to_string(), "dashboard".to_string(),
            "wp-admin".to_string(), "backup".to_string(), ".env".to_string(),
            "config".to_string(), "api".to_string(), "v1".to_string(),
            "test".to_string(), "dev".to_string(), "staging".to_string(),
            "phpmyadmin".to_string(), "cgi-bin".to_string(), "robots.txt".to_string(),
            "sitemap.xml".to_string(),
        ]
    }
}

/// Build an HTTP client with optional proxy, timeout, and user‑agent
pub fn build_http_client(proxy: Option<&str>, timeout_secs: u64, user_agent: &str) -> reqwest::Client {
    let mut builder = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(timeout_secs))
        .user_agent(user_agent);
    if let Some(proxy_url) = proxy {
        if let Ok(proxy) = reqwest::Proxy::all(proxy_url) {
            builder = builder.proxy(proxy);
        }
    }
    builder.build().expect("Failed to build HTTP client")
}