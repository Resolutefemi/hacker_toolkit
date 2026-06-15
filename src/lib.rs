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
pub use scanner::{ScanResult, ScannerConfig, ScanType};
pub use utils::{create_rate_limiter, load_wordlist, build_http_client, SharedRateLimiter};