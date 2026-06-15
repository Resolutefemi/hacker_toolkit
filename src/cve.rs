//! Local CVE database for product version matching

use reqwest::Client;
use url::Url;
use once_cell::sync::Lazy;

/// Database of (product substring, CVE ID, description)
static CVE_DB: Lazy<Vec<(String, String, String)>> = Lazy::new(|| {
    vec![
        ("nginx/1.14".to_string(), "CVE-2021-23017".to_string(), "nginx 1.14.x vulnerability".to_string()),
        ("nginx/1.16".to_string(), "CVE-2021-23017".to_string(), "nginx 1.16.x vulnerability".to_string()),
        ("Apache/2.4.49".to_string(), "CVE-2021-41773".to_string(), "Apache 2.4.49 path traversal".to_string()),
        ("Apache/2.4.50".to_string(), "CVE-2021-42013".to_string(), "Apache 2.4.50 RCE".to_string()),
        ("OpenSSL/1.1.1".to_string(), "CVE-2022-0778".to_string(), "OpenSSL 1.1.1 infinite loop".to_string()),
        ("OpenSSL/3.0.0".to_string(), "CVE-2022-0778".to_string(), "OpenSSL 3.0.0 infinite loop".to_string()),
        ("PHP/7.4".to_string(), "CVE-2024-1234".to_string(), "PHP 7.4 vulnerability (simulated)".to_string()),
        ("PHP/8.1".to_string(), "CVE-2023-4567".to_string(), "PHP 8.1 buffer overflow".to_string()),
        ("IIS/10.0".to_string(), "CVE-2021-31274".to_string(), "IIS 10.0 HTTP/2 vulnerability".to_string()),
        ("IIS/8.5".to_string(), "CVE-2017-7269".to_string(), "IIS 8.5 WebDAV vulnerability".to_string()),
        ("Node.js".to_string(), "CVE-2023-30584".to_string(), "Node.js 20.x prototype pollution".to_string()),
    ]
});

/// Check for known CVEs based on Server header or other banners
pub async fn check_cves(client: &Client, base_url: &Url) -> Vec<String> {
    let mut matches = Vec::new();
    
    // Try to get Server header
    if let Ok(resp) = client.get(base_url.clone()).send().await {
        if let Some(server) = resp.headers().get("server") {
            let server_str = server.to_str().unwrap_or("");
            for (prod, cve, desc) in CVE_DB.iter() {
                if server_str.to_lowercase().contains(&prod.to_lowercase()) {
                    matches.push(format!("{} → {}: {}", prod, cve, desc));
                }
            }
        }
        
        // Try to get X-Powered-By header
        if let Some(powered) = resp.headers().get("x-powered-by") {
            let powered_str = powered.to_str().unwrap_or("");
            for (prod, cve, desc) in CVE_DB.iter() {
                if powered_str.to_lowercase().contains(&prod.to_lowercase()) {
                    matches.push(format!("{} → {}: {}", prod, cve, desc));
                }
            }
        }
    }
    
    matches
}

/// Manual check for a specific product version (for use in other modules)
pub fn check_product_cve(product: &str, version: &str) -> Vec<String> {
    let full = format!("{}/{}", product, version);
    let mut results = Vec::new();
    for (prod, cve, desc) in CVE_DB.iter() {
        if full.to_lowercase().contains(&prod.to_lowercase()) {
            results.push(format!("{} → {}: {}", prod, cve, desc));
        }
    }
    results
}