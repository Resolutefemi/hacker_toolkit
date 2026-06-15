//! Local CVE (Common Vulnerabilities and Exposures) database
//! Contains known vulnerabilities for common software versions.
//! Used by the scanner to match banners and server headers.

use reqwest::Client;
use url::Url;
use once_cell::sync::Lazy;
use serde::{Serialize, Deserialize};

/// CVE entry structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CveEntry {
    pub id: String,
    pub product: String,
    pub version_affected: String,
    pub description: String,
    pub cvss_score: f32,
    pub published_year: u16,
}

/// Full CVE database
pub static CVE_DB: Lazy<Vec<CveEntry>> = Lazy::new(|| {
    vec![
        // Web servers
        CveEntry { id: "CVE-2017-5638".to_string(), product: "Apache Struts".to_string(), version_affected: "2.3.5 - 2.3.31".to_string(), description: "Remote code execution via Content-Type header".to_string(), cvss_score: 10.0, published_year: 2017 },
        CveEntry { id: "CVE-2021-41773".to_string(), product: "Apache HTTP Server".to_string(), version_affected: "2.4.49".to_string(), description: "Path traversal and RCE".to_string(), cvss_score: 9.8, published_year: 2021 },
        CveEntry { id: "CVE-2021-42013".to_string(), product: "Apache HTTP Server".to_string(), version_affected: "2.4.50".to_string(), description: "Path traversal and RCE (bypass)".to_string(), cvss_score: 9.8, published_year: 2021 },
        CveEntry { id: "CVE-2019-0230".to_string(), product: "Apache Struts".to_string(), version_affected: "2.0.0 - 2.5.20".to_string(), description: "RCE via OGNL injection".to_string(), cvss_score: 9.8, published_year: 2019 },
        CveEntry { id: "CVE-2020-1938".to_string(), product: "Apache Tomcat".to_string(), version_affected: "6.x, 7.x, 8.x, 9.x".to_string(), description: "Ghostcat file read".to_string(), cvss_score: 9.8, published_year: 2020 },
        CveEntry { id: "CVE-2021-44228".to_string(), product: "Apache Log4j".to_string(), version_affected: "2.0 - 2.14.1".to_string(), description: "Log4Shell RCE".to_string(), cvss_score: 10.0, published_year: 2021 },
        CveEntry { id: "CVE-2021-45046".to_string(), product: "Apache Log4j".to_string(), version_affected: "2.15.0".to_string(), description: "Denial of service and RCE".to_string(), cvss_score: 9.0, published_year: 2021 },
        
        // NGINX
        CveEntry { id: "CVE-2021-23017".to_string(), product: "nginx".to_string(), version_affected: "1.14.x - 1.20.x".to_string(), description: "Request smuggling vulnerability".to_string(), cvss_score: 7.5, published_year: 2021 },
        CveEntry { id: "CVE-2017-7529".to_string(), product: "nginx".to_string(), version_affected: "0.5.6 - 1.13.2".to_string(), description: "Integer overflow in range filter".to_string(), cvss_score: 7.5, published_year: 2017 },
        CveEntry { id: "CVE-2013-2028".to_string(), product: "nginx".to_string(), version_affected: "1.3.9 - 1.4.0".to_string(), description: "Stack-based buffer overflow".to_string(), cvss_score: 7.5, published_year: 2013 },
        
        // OpenSSL
        CveEntry { id: "CVE-2014-0160".to_string(), product: "OpenSSL".to_string(), version_affected: "1.0.1 - 1.0.1f".to_string(), description: "Heartbleed information leak".to_string(), cvss_score: 7.5, published_year: 2014 },
        CveEntry { id: "CVE-2022-0778".to_string(), product: "OpenSSL".to_string(), version_affected: "1.1.1 - 1.1.1n".to_string(), description: "Infinite loop in BN_mod_sqrt".to_string(), cvss_score: 7.5, published_year: 2022 },
        CveEntry { id: "CVE-2016-2107".to_string(), product: "OpenSSL".to_string(), version_affected: "1.0.2 - 1.0.2h".to_string(), description: "AES-NI padding oracle".to_string(), cvss_score: 5.9, published_year: 2016 },
        
        // PHP
        CveEntry { id: "CVE-2019-11043".to_string(), product: "PHP".to_string(), version_affected: "7.3.x - 7.4.x".to_string(), description: "FastCGI RCE (PHP-FPM)".to_string(), cvss_score: 9.8, published_year: 2019 },
        CveEntry { id: "CVE-2024-4577".to_string(), product: "PHP".to_string(), version_affected: "8.1 - 8.3".to_string(), description: "Argument injection in CGI mode".to_string(), cvss_score: 9.8, published_year: 2024 },
        CveEntry { id: "CVE-2015-4598".to_string(), product: "PHP".to_string(), version_affected: "5.6.x".to_string(), description: "Type confusion in unserialize".to_string(), cvss_score: 7.5, published_year: 2015 },
        
        // WordPress
        CveEntry { id: "CVE-2024-43688".to_string(), product: "WordPress".to_string(), version_affected: "< 6.5.5".to_string(), description: "Object injection in PHPMailer".to_string(), cvss_score: 8.8, published_year: 2024 },
        CveEntry { id: "CVE-2023-3460".to_string(), product: "WordPress".to_string(), version_affected: "< 6.2.2".to_string(), description: "Unauthenticated RCE via plugin".to_string(), cvss_score: 9.8, published_year: 2023 },
        CveEntry { id: "CVE-2017-8295".to_string(), product: "WordPress".to_string(), version_affected: "< 4.7.4".to_string(), description: "Password reset spoofing".to_string(), cvss_score: 7.5, published_year: 2017 },
        
        // Microsoft IIS
        CveEntry { id: "CVE-2021-31274".to_string(), product: "IIS".to_string(), version_affected: "10.0".to_string(), description: "HTTP/2 request smuggling".to_string(), cvss_score: 7.5, published_year: 2021 },
        CveEntry { id: "CVE-2017-7269".to_string(), product: "IIS".to_string(), version_affected: "6.0, 7.5".to_string(), description: "WebDAV buffer overflow".to_string(), cvss_score: 9.8, published_year: 2017 },
        CveEntry { id: "CVE-2015-1635".to_string(), product: "IIS".to_string(), version_affected: "7.0, 7.5, 8.0, 8.5".to_string(), description: "HTTP.sys RCE (MS15-034)".to_string(), cvss_score: 9.8, published_year: 2015 },
        
        // Node.js
        CveEntry { id: "CVE-2023-30584".to_string(), product: "Node.js".to_string(), version_affected: "20.x".to_string(), description: "Prototype pollution in fetch API".to_string(), cvss_score: 8.8, published_year: 2023 },
        CveEntry { id: "CVE-2022-21824".to_string(), product: "Node.js".to_string(), version_affected: "< 17.3.0".to_string(), description: "Bypass of allowedHosts in inspector".to_string(), cvss_score: 7.5, published_year: 2022 },
        CveEntry { id: "CVE-2020-8265".to_string(), product: "Node.js".to_string(), version_affected: "< 14.15.4".to_string(), description: "Denial of service via request smuggling".to_string(), cvss_score: 7.5, published_year: 2020 },
        
        // MySQL / MariaDB
        CveEntry { id: "CVE-2022-21546".to_string(), product: "MySQL".to_string(), version_affected: "8.0.30".to_string(), description: "Privilege escalation".to_string(), cvss_score: 8.0, published_year: 2022 },
        CveEntry { id: "CVE-2020-2922".to_string(), product: "MySQL".to_string(), version_affected: "8.0.19".to_string(), description: "DoS vulnerability".to_string(), cvss_score: 7.5, published_year: 2020 },
        CveEntry { id: "CVE-2018-2696".to_string(), product: "MySQL".to_string(), version_affected: "5.7.21".to_string(), description: "Privilege escalation in client".to_string(), cvss_score: 7.3, published_year: 2018 },
        
        // Redis
        CveEntry { id: "CVE-2022-0543".to_string(), product: "Redis".to_string(), version_affected: "5.x, 6.x, 7.x".to_string(), description: "Debian-specific Lua sandbox escape".to_string(), cvss_score: 9.8, published_year: 2022 },
        CveEntry { id: "CVE-2019-10192".to_string(), product: "Redis".to_string(), version_affected: "< 5.0.6".to_string(), description: "Memory corruption in hyperloglog".to_string(), cvss_score: 7.5, published_year: 2019 },
        
        // MongoDB
        CveEntry { id: "CVE-2021-32022".to_string(), product: "MongoDB".to_string(), version_affected: "4.4.x".to_string(), description: "Denial of service via malformed SSL packets".to_string(), cvss_score: 7.5, published_year: 2021 },
        CveEntry { id: "CVE-2019-20925".to_string(), product: "MongoDB".to_string(), version_affected: "< 4.2.1".to_string(), description: "Unauthenticated client side disconnection".to_string(), cvss_score: 7.5, published_year: 2019 },
        
        // Docker
        CveEntry { id: "CVE-2021-41091".to_string(), product: "Docker".to_string(), version_affected: "20.10.9".to_string(), description: "Container breakout via symlinks".to_string(), cvss_score: 7.5, published_year: 2021 },
        CveEntry { id: "CVE-2020-15257".to_string(), product: "Docker".to_string(), version_affected: "19.03.x".to_string(), description: "Containerd API exposure".to_string(), cvss_score: 7.0, published_year: 2020 },
        
        // Kubernetes
        CveEntry { id: "CVE-2020-8554".to_string(), product: "Kubernetes".to_string(), version_affected: "1.19.x".to_string(), description: "Man-in-the-middle via LoadBalancer".to_string(), cvss_score: 6.5, published_year: 2020 },
        CveEntry { id: "CVE-2019-11246".to_string(), product: "Kubernetes".to_string(), version_affected: "1.12.x".to_string(), description: "Privilege escalation via kubelet".to_string(), cvss_score: 8.8, published_year: 2019 },
    ]
});

/// Search CVE database by product name (case-insensitive)
pub fn search_by_product(product: &str) -> Vec<&CveEntry> {
    let product_lower = product.to_lowercase();
    CVE_DB.iter()
        .filter(|cve| cve.product.to_lowercase().contains(&product_lower))
        .collect()
}

/// Search by version string (simple string contains)
pub fn search_by_version(version: &str) -> Vec<&CveEntry> {
    let version_lower = version.to_lowercase();
    CVE_DB.iter()
        .filter(|cve| cve.version_affected.to_lowercase().contains(&version_lower))
        .collect()
}

/// Get all CVE entries
pub fn get_all_cves() -> &'static Vec<CveEntry> {
    &CVE_DB
}

/// Check for CVE matches using server header or banner
pub async fn check_cves(client: &Client, base_url: &Url) -> Vec<String> {
    let mut matches = Vec::new();
    
    // Try to get Server header
    if let Ok(resp) = client.get(base_url.clone()).send().await {
        if let Some(server) = resp.headers().get("server") {
            let server_str = server.to_str().unwrap_or("").to_lowercase();
            for cve in CVE_DB.iter() {
                if server_str.contains(&cve.product.to_lowercase()) {
                    matches.push(format!("{} - {}: {} (CVSS: {})", cve.id, cve.product, cve.description, cve.cvss_score));
                }
            }
        }
        
        // Try X-Powered-By header
        if let Some(powered) = resp.headers().get("x-powered-by") {
            let powered_str = powered.to_str().unwrap_or("").to_lowercase();
            for cve in CVE_DB.iter() {
                if powered_str.contains(&cve.product.to_lowercase()) {
                    matches.push(format!("{} - {}: {} (CVSS: {})", cve.id, cve.product, cve.description, cve.cvss_score));
                }
            }
        }
    }
    matches
}

/// Manual CVE check for any product version
pub fn check_product_cve(product: &str, version: &str) -> Vec<String> {
    let product_lower = product.to_lowercase();
    let version_lower = version.to_lowercase();
    let mut results = Vec::new();
    for cve in CVE_DB.iter() {
        if cve.product.to_lowercase().contains(&product_lower) && 
           cve.version_affected.to_lowercase().contains(&version_lower) {
            results.push(format!("{} - {}: {} (CVSS: {})", cve.id, cve.product, cve.description, cve.cvss_score));
        }
    }
    results
}

/// Search CVE database using a general query keyword
pub fn search_cves(query: &str) -> Vec<CveEntry> {
    let query_lower = query.to_lowercase();
    CVE_DB.iter()
        .filter(|cve| {
            cve.id.to_lowercase().contains(&query_lower) ||
            cve.product.to_lowercase().contains(&query_lower) ||
            cve.description.to_lowercase().contains(&query_lower) ||
            cve.version_affected.to_lowercase().contains(&query_lower)
        })
        .cloned()
        .collect()
}