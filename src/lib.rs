//! Core Library
//! Exports all modules for use in CLI and GUI.

pub mod scanner;
pub mod stress;
pub mod credential;
pub mod spam;
pub mod payload;
pub mod report;
pub mod utils;
pub mod cve;

// Re-export commonly used types
pub use scanner::{ScanResult, ScannerConfig, ScanType, run_full_scan};
pub use utils::{create_rate_limiter, load_wordlist, build_http_client, SharedRateLimiter, load_proxy_list};
pub use credential::{CredStuffConfig, LoginResult, credential_stuffing, save_successful_logins, load_wordlist_from_file};
pub use payload::{Platform, generate_reverse_shell, generate_bind_shell, random_webshell_password, generate_php_webshell, generate_download_exec};
pub use report::{save_html_report, save_json_report, generate_combined_report};
pub use spam::{flood_database, email_bomber, sms_bomber, comment_spam, registration_spam};