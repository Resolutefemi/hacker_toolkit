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
    pub technologies: Vec<String>,
    pub subdomain_takeovers: Vec<String>,
    pub zone_transfers: Vec<String>,
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
            technologies: Vec::new(),
            subdomain_takeovers: Vec::new(),
            zone_transfers: Vec::new(),
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
    use futures::future::join_all;
    let mut tasks = Vec::new();
    for &port in ports {
        let host = host.to_string();
        let limiter = limiter.clone();
        tasks.push(tokio::spawn(async move {
            throttle(&limiter).await;
            let addr = format!("{}:{}", host, port);
            if let Ok(Ok(_stream)) = tokio::time::timeout(
                std::time::Duration::from_millis(1500),
                tokio::net::TcpStream::connect(&addr)
            ).await {
                Some((port, get_service_name(port).to_string()))
            } else {
                None
            }
        }));
    }
    let results = join_all(tasks).await;
    let mut open = Vec::new();
    for res in results {
        if let Ok(Some(item)) = res {
            open.push(item);
        }
    }
    open.sort_by_key(|&(p, _)| p);
    open
}

// ========== Directory Bruteforce ==========
pub async fn dir_bruteforce(
    client: &Client,
    base_url: &Url,
    wordlist: &[String],
    limiter: &SharedRateLimiter,
) -> Vec<String> {
    use futures::future::join_all;
    let mut tasks = Vec::new();
    for path in wordlist {
        let client = client.clone();
        let base_url = base_url.clone();
        let path = path.clone();
        let limiter = limiter.clone();
        tasks.push(tokio::spawn(async move {
            throttle(&limiter).await;
            let url = match base_url.join(&path) {
                Ok(u) => u,
                Err(_) => return None,
            };
            match client.get(url.clone()).send().await {
                Ok(resp) => {
                    if resp.status().is_success() {
                        Some(url.to_string())
                    } else {
                        None
                    }
                }
                Err(_) => None,
            }
        }));
    }
    let results = join_all(tasks).await;
    let mut found = Vec::new();
    for res in results {
        if let Ok(Some(url)) = res {
            found.push(url);
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

    use futures::future::join_all;
    let mut tasks = Vec::new();
    for param in params {
        for payload in &payloads {
            let client = client.clone();
            let base_url = base_url.clone();
            let param = param.to_string();
            let payload = payload.to_string();
            let limiter = limiter.clone();
            tasks.push(tokio::spawn(async move {
                throttle(&limiter).await;
                let mut url = base_url.clone();
                url.query_pairs_mut().append_pair(&param, &payload);
                match client.get(url.clone()).send().await {
                    Ok(resp) => {
                        let body = resp.text().await.unwrap_or_default().to_lowercase();
                        if body.contains("sql") || body.contains("mysql") || body.contains("syntax")
                            || body.contains("unclosed") || body.contains("odbc")
                            || body.contains("oracle") || body.contains("postgresql") {
                            Some(url.to_string())
                        } else {
                            None
                        }
                    }
                    Err(_) => None,
                }
            }));
        }
    }
    let results = join_all(tasks).await;
    let mut vuln = Vec::new();
    for res in results {
        if let Ok(Some(url)) = res {
            vuln.push(url);
        }
    }
    vuln.dedup();
    vuln
}

// ========== XSS ==========
pub async fn xss_test(client: &Client, base_url: &Url, limiter: &SharedRateLimiter) -> Vec<String> {
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

    use futures::future::join_all;
    let mut tasks = Vec::new();
    for param in params {
        for payload in &payloads {
            let client = client.clone();
            let base_url = base_url.clone();
            let param = param.to_string();
            let payload = payload.to_string();
            let limiter = limiter.clone();
            tasks.push(tokio::spawn(async move {
                throttle(&limiter).await;
                let mut url = base_url.clone();
                url.query_pairs_mut().append_pair(&param, &payload);
                match client.get(url.clone()).send().await {
                    Ok(resp) => {
                        let body = resp.text().await.unwrap_or_default().to_lowercase();
                        if body.contains("<script") || body.contains("onerror")
                            || body.contains("alert") || body.contains("javascript:")
                            || body.contains("confirm") {
                            Some(url.to_string())
                        } else {
                            None
                        }
                    }
                    Err(_) => None,
                }
            }));
        }
    }
    let results = join_all(tasks).await;
    let mut vuln = Vec::new();
    for res in results {
        if let Ok(Some(url)) = res {
            vuln.push(url);
        }
    }
    vuln.dedup();
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
    use futures::future::join_all;
    let mut tasks = Vec::new();
    for prefix in prefixes {
        let domain = domain.to_string();
        let limiter = limiter.clone();
        tasks.push(tokio::spawn(async move {
            throttle(&limiter).await;
            let sub = format!("{}.{}", prefix, domain);
            let addr = format!("{}:80", sub);
            if let Ok(Ok(_stream)) = tokio::time::timeout(
                std::time::Duration::from_millis(1500),
                tokio::net::TcpStream::connect(&addr)
            ).await {
                Some(sub)
            } else {
                None
            }
        }));
    }
    let results = join_all(tasks).await;
    let mut found = Vec::new();
    for res in results {
        if let Ok(Some(sub)) = res {
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
    // WAF Detection
    let waf_results = detect_waf(&client, &base_url).await;
    if !waf_results.is_empty() {
        result.technologies.push("[WAF] ----".to_string());
        result.technologies.extend(waf_results);
    }

    // 7. Security headers
    result.security_headers = get_security_headers(&client, &base_url).await;
    // 8. CVE matches
    result.cve_matches = cve::check_cves(&client, &base_url).await;
    
    // 9. Technology Fingerprinting
    result.technologies = fingerprint_technologies(&client, &base_url, &result.security_headers).await;

    // 10. Subdomain Takeovers
    if !result.subdomains.is_empty() {
        result.subdomain_takeovers = detect_takeovers(&client, &result.subdomains).await;
    }

    // 11. DNS Zone Transfer
    result.zone_transfers = check_zone_transfer(&host).await;

    if let Some(cb) = &progress_callback { cb(1.0); }
    result
}

// ========== Advanced Technology Fingerprinting ==========
pub async fn fingerprint_technologies(client: &Client, base_url: &Url, headers_str: &str) -> Vec<String> {
    let mut techs = Vec::new();
    let lower_headers = headers_str.to_lowercase();

    // ===== SERVER & OS DETECTION =====
    if lower_headers.contains("server: apache") { techs.push("Apache HTTP Server".to_string()); }
    if lower_headers.contains("server: nginx") || lower_headers.contains("server: nginx/") { techs.push("Nginx Web Server".to_string()); }
    if lower_headers.contains("server: cloudflare") { techs.push("Cloudflare CDN/WAF".to_string()); }
    if lower_headers.contains("server: microsoft-iis") || lower_headers.contains("server: iis") { techs.push("Microsoft IIS Web Server".to_string()); }
    if lower_headers.contains("server: caddy") { techs.push("Caddy Web Server".to_string()); }
    if lower_headers.contains("server: lighttpd") { techs.push("Lighttpd Web Server".to_string()); }
    if lower_headers.contains("server: tomcat") { techs.push("Apache Tomcat".to_string()); }
    if lower_headers.contains("server: jetty") { techs.push("Eclipse Jetty".to_string()); }
    if lower_headers.contains("server: gunicorn") { techs.push("Gunicorn Python WSGI".to_string()); }
    if lower_headers.contains("server: uwsgi") { techs.push("uWSGI Server".to_string()); }
    if lower_headers.contains("server: openresty") { techs.push("OpenResty (Nginx + Lua)".to_string()); }
    if lower_headers.contains("via: 1.1 varnish") || lower_headers.contains("x-varnish") { techs.push("Varnish Cache".to_string()); }
    if lower_headers.contains("x-powered-by: asp.net") || lower_headers.contains("x-aspnet-version") { techs.push("ASP.NET Framework".to_string()); }
    // OS
    if lower_headers.contains("server: ubuntu") || lower_headers.contains("server: debian") { techs.push("Linux (Ubuntu/Debian)".to_string()); }
    if lower_headers.contains("server: centos") || lower_headers.contains("server: red hat") { techs.push("Linux (RHEL/CentOS)".to_string()); }
    if lower_headers.contains("server: windows") { techs.push("Microsoft Windows Server".to_string()); }
    if lower_headers.contains("server: freebsd") { techs.push("FreeBSD Unix".to_string()); }

    // ===== BACKEND LANGUAGES =====
    if lower_headers.contains("x-powered-by: php") { techs.push("PHP Backend".to_string()); }
    if lower_headers.contains("x-powered-by: express") { techs.push("Express.js (Node.js)".to_string()); }
    if lower_headers.contains("x-powered-by: rails") || lower_headers.contains("x-rails") { techs.push("Ruby on Rails".to_string()); }
    if lower_headers.contains("x-powered-by: django") { techs.push("Django (Python)".to_string()); }
    if lower_headers.contains("x-powered-by: flask") { techs.push("Flask (Python)".to_string()); }
    if lower_headers.contains("x-powered-by: laravel") { techs.push("Laravel (PHP)".to_string()); }
    if lower_headers.contains("x-powered-by: next.js") { techs.push("Next.js Framework".to_string()); }
    if lower_headers.contains("x-generator: drupal") { techs.push("Drupal CMS".to_string()); }
    if lower_headers.contains("x-ghost-cache") { techs.push("Ghost CMS".to_string()); }

    // ===== CLOUD & HOSTING =====
    if lower_headers.contains("cf-ray") { techs.push("Cloudflare CDN/Proxy".to_string()); }
    if lower_headers.contains("x-amz-") { techs.push("Amazon Web Services (AWS)".to_string()); }
    if lower_headers.contains("x-azure-") || lower_headers.contains("azure-ref") { techs.push("Microsoft Azure".to_string()); }
    if lower_headers.contains("x-goog-") || lower_headers.contains("x-guploader") { techs.push("Google Cloud Platform (GCP)".to_string()); }
    if lower_headers.contains("x-fastly-") { techs.push("Fastly CDN".to_string()); }
    if lower_headers.contains("x-akamai-") { techs.push("Akamai CDN".to_string()); }
    if lower_headers.contains("x-vercel-") { techs.push("Vercel (Serverless)".to_string()); }
    if lower_headers.contains("x-nf-request-id") { techs.push("Netlify".to_string()); }
    if lower_headers.contains("via: 1.1 vegur") { techs.push("Heroku".to_string()); }

    // ===== WAF DETECTION =====
    if lower_headers.contains("cf-ray") || lower_headers.contains("__cfduid") { techs.push("Cloudflare WAF".to_string()); }
    if lower_headers.contains("x-sucuri-id") { techs.push("Sucuri WAF".to_string()); }
    if lower_headers.contains("x-mod-security") || lower_headers.contains("mod_security") { techs.push("ModSecurity WAF".to_string()); }
    if lower_headers.contains("x-waf") || lower_headers.contains("x-aws-waf") { techs.push("AWS WAF".to_string()); }
    if lower_headers.contains("x-imperva-") || lower_headers.contains("incapsula") { techs.push("Imperva / Incapsula WAF".to_string()); }
    if lower_headers.contains("x-barracuda-") { techs.push("Barracuda WAF".to_string()); }
    if lower_headers.contains("x-f5-") || lower_headers.contains("big-ip") { techs.push("F5 BIG-IP WAF".to_string()); }
    if lower_headers.contains("x-fortinet-") || lower_headers.contains("fortigate") { techs.push("Fortinet FortiWeb WAF".to_string()); }

    // Fetch homepage content for deeper analysis
    if let Ok(resp) = client.get(base_url.as_str()).send().await {
        let headers = resp.headers();
        if let Some(cookie) = headers.get("set-cookie") {
            let c = cookie.to_str().unwrap_or("").to_lowercase();
            if c.contains("asp.net_sessionid") { techs.push("ASP.NET Session".to_string()); }
            if c.contains("laravel_session") { techs.push("Laravel (PHP)".to_string()); }
            if c.contains("ci_session") { techs.push("CodeIgniter (PHP)".to_string()); }
            if c.contains("symfony") { techs.push("Symfony (PHP)".to_string()); }
            if c.contains("wordpress_") || c.contains("wp-") { techs.push("WordPress CMS".to_string()); }
            if c.contains("drupal") { techs.push("Drupal CMS".to_string()); }
            if c.contains("magento") || c.contains("mage-") { techs.push("Magento CMS".to_string()); }
            if c.contains("shopify") { techs.push("Shopify".to_string()); }
            if c.contains("_ga") { techs.push("Google Analytics".to_string()); }
            if c.contains("_fbp") { techs.push("Facebook Pixel".to_string()); }
            if c.contains("_hjid") { techs.push("Hotjar Analytics".to_string()); }
            if c.contains("mp_") { techs.push("Mixpanel Analytics".to_string()); }
        }

        if let Ok(body) = resp.text().await {
            let lb = body.to_lowercase();

            // ===== CMS =====
            if lb.contains("/wp-content/") || lb.contains("/wp-includes/") {
                techs.push("WordPress CMS".to_string());
                if let Some(pos) = body.find("?ver=") {
                    let start = pos + 5;
                    let end = std::cmp::min(start + 15, body.len());
                    let ver: String = body[start..end].chars().take_while(|c| c.is_alphanumeric() || *c == '.' || *c == '-').collect();
                    if ver.len() >= 2 { techs.push(format!("WordPress v{}", ver)); }
                }
            }
            if lb.contains("joomla") || lb.contains("/components/com_") { techs.push("Joomla! CMS".to_string()); }
            if lb.contains("drupal") || lb.contains("/sites/default/") { techs.push("Drupal CMS".to_string()); }
            if lb.contains("magento") || lb.contains("/static/version") { techs.push("Magento/Adobe Commerce".to_string()); }
            if lb.contains("shopify") || lb.contains("myshopify") { techs.push("Shopify Store".to_string()); }
            if lb.contains("squarespace") { techs.push("Squarespace CMS".to_string()); }
            if lb.contains("wix.com") || lb.contains("wixstatic") { techs.push("Wix Website Builder".to_string()); }
            if lb.contains("weebly.com") { techs.push("Weebly Site Builder".to_string()); }
            if lb.contains("prestashop") { techs.push("PrestaShop E-commerce".to_string()); }
            if lb.contains("opencart") || lb.contains("oc_theme") { techs.push("OpenCart E-commerce".to_string()); }
            if lb.contains("woocommerce") || lb.contains("/wp-json/wc/") { techs.push("WooCommerce (WordPress)".to_string()); }
            if lb.contains("hubspot") || lb.contains("hs-analytics") { techs.push("HubSpot CMS".to_string()); }
            if lb.contains("webflow") { techs.push("Webflow CMS".to_string()); }
            if lb.contains("ghost") && (lb.contains("ghost-cache") || lb.contains("ghost.io")) { techs.push("Ghost Blog CMS".to_string()); }

            // ===== JS FRAMEWORKS =====
            if lb.contains("_next/static") || lb.contains("__next") { techs.push("Next.js (React)".to_string()); }
            if lb.contains("__nuxt") || lb.contains("_nuxt/") { techs.push("Nuxt.js (Vue)".to_string()); }
            if lb.contains("ng-version") || lb.contains("ng-app") { techs.push("Angular".to_string()); }
            if lb.contains("react") && (lb.contains("react-dom") || lb.contains("__react")) { techs.push("React.js".to_string()); }
            if lb.contains("vue") && (lb.contains("vue-") || lb.contains("v-cloak")) { techs.push("Vue.js".to_string()); }
            if lb.contains("svelte") && (lb.contains("svelte-") || lb.contains("sveltekit")) { techs.push("Svelte / SvelteKit".to_string()); }
            if lb.contains("remix") || lb.contains("__remix") { techs.push("Remix Framework".to_string()); }
            if lb.contains("gatsby") { techs.push("Gatsby SSG".to_string()); }
            if lb.contains("astro") { techs.push("Astro Framework".to_string()); }
            if lb.contains("solid-js") || lb.contains("solidjs") { techs.push("Solid.js".to_string()); }
            if lb.contains("qwik") { techs.push("Qwik Framework".to_string()); }
            if lb.contains("11ty") || lb.contains("eleventy") { techs.push("Eleventy (11ty) SSG".to_string()); }
            if lb.contains("jekyll") { techs.push("Jekyll SSG".to_string()); }
            if lb.contains("hugo") || lb.contains("gohugo") { techs.push("Hugo SSG".to_string()); }

            // ===== CSS FRAMEWORKS =====
            if lb.contains("tailwind") || lb.contains("tailwindcss") { techs.push("Tailwind CSS".to_string()); }
            if lb.contains("bootstrap") && lb.contains(".bootstrap") { techs.push("Bootstrap CSS".to_string()); }
            if lb.contains("bulma") || lb.contains("bulma-") { techs.push("Bulma CSS".to_string()); }
            if lb.contains("materialize") { techs.push("Materialize CSS".to_string()); }
            if lb.contains("foundation") && lb.contains("foundation.") { techs.push("Foundation CSS".to_string()); }

            // ===== BUILD TOOLS =====
            if lb.contains("webpack") || lb.contains("__webpack") { techs.push("Webpack Bundler".to_string()); }
            if lb.contains("vite") || lb.contains("/@vite/") { techs.push("Vite Build Tool".to_string()); }
            if lb.contains("parcel") { techs.push("Parcel Bundler".to_string()); }
            if lb.contains("esbuild") { techs.push("esbuild Bundler".to_string()); }

            // ===== ANALYTICS & MONITORING =====
            if lb.contains("google-analytics") || lb.contains("gtag") || lb.contains("ga.js") { techs.push("Google Analytics".to_string()); }
            if lb.contains("googletagmanager") || lb.contains("gtm.js") { techs.push("Google Tag Manager".to_string()); }
            if lb.contains("facebook") && lb.contains("pixel") { techs.push("Facebook Pixel".to_string()); }
            if lb.contains("hotjar") { techs.push("Hotjar Analytics".to_string()); }
            if lb.contains("mixpanel") { techs.push("Mixpanel Analytics".to_string()); }
            if lb.contains("amplitude") { techs.push("Amplitude Analytics".to_string()); }
            if lb.contains("segment") || lb.contains("segment.io") { techs.push("Segment Analytics".to_string()); }
            if lb.contains("fullstory") { techs.push("FullStory Session Replay".to_string()); }
            if lb.contains("intercom") { techs.push("Intercom Chat".to_string()); }
            if lb.contains("drift") { techs.push("Drift Chat".to_string()); }
            if lb.contains("crisp") { techs.push("Crisp Chat".to_string()); }
            if lb.contains("tawk.to") { techs.push("Tawk.to Chat".to_string()); }
            if lb.contains("zendesk") || lb.contains("zopim") { techs.push("Zendesk Support".to_string()); }

            // ===== PAYMENTS =====
            if lb.contains("stripe") || lb.contains("js/stripe") { techs.push("Stripe Payments".to_string()); }
            if lb.contains("paypal") || lb.contains("paypal.com") { techs.push("PayPal Payments".to_string()); }
            if lb.contains("squareup") || lb.contains("square.com") { techs.push("Square Payments".to_string()); }
            if lb.contains("braintree") { techs.push("Braintree Payments".to_string()); }

            // ===== CDN & FONTS =====
            if lb.contains("/cdn-cgi/") { techs.push("cdnjs (Cloudflare)".to_string()); }
            if lb.contains("jsdelivr") { techs.push("jsDelivr CDN".to_string()); }
            if lb.contains("unpkg.com") { techs.push("Unpkg CDN".to_string()); }
            if lb.contains("fonts.googleapis") { techs.push("Google Fonts".to_string()); }
            if lb.contains("font-awesome") || lb.contains("fontawesome") { techs.push("Font Awesome Icons".to_string()); }

            // ===== AUTH & SECURITY =====
            if lb.contains("auth0") { techs.push("Auth0 Authentication".to_string()); }
            if lb.contains("okta") { techs.push("Okta SSO".to_string()); }
            if lb.contains("firebase") || lb.contains("firebaseio") { techs.push("Firebase (Google)".to_string()); }
            if lb.contains("recaptcha") || lb.contains("google.com/recaptcha") { techs.push("reCAPTCHA Anti-bot".to_string()); }
            if lb.contains("hcaptcha") { techs.push("hCaptcha Anti-bot".to_string()); }
        }
    }
    techs.dedup();
    techs
}

// ========== WAF Detection ==========
pub async fn detect_waf(client: &Client, base_url: &Url) -> Vec<String> {
    let mut wafs = Vec::new();
    for payload in &["../../../etc/passwd", "<script>alert(1)</script>", "1' OR '1'='1"] {
        let mut url = base_url.clone();
        url.query_pairs_mut().append_pair("test", payload);
        if let Ok(resp) = client.get(url).send().await {
            let status = resp.status().as_u16();
            if status == 403 || status == 406 || status == 429 || status == 503 {
                let h = format!("{:?}", resp.headers()).to_lowercase();
                if h.contains("cf-ray") { wafs.push("Cloudflare WAF (triggered)".to_string()); }
                if h.contains("x-sucuri") { wafs.push("Sucuri WAF (triggered)".to_string()); }
                if h.contains("mod_security") { wafs.push("ModSecurity WAF (triggered)".to_string()); }
            }
        }
    }
    wafs.dedup();
    wafs
}

// ========== WAF Detection ==========

// ========== DNS Zone Transfer (AXFR) Check ==========
pub async fn check_zone_transfer(host: &str) -> Vec<String> {
    let mut vulnerabilities = Vec::new();
    let domain = crate::utils::extract_domain(host);
    
    // Create hickory resolver
    let resolver = hickory_resolver::TokioAsyncResolver::tokio(
        hickory_resolver::config::ResolverConfig::default(),
        hickory_resolver::config::ResolverOpts::default(),
    );

    // Lookup NS records
    if let Ok(ns_lookup) = resolver.ns_lookup(format!("{}.", domain)).await {
        for ns in ns_lookup.iter() {
            let ns_name = ns.to_string();
            // Resolve Name Server IP
            if let Ok(ips) = resolver.lookup_ip(&ns_name).await {
                for ip in ips.iter() {
                    // Try to connect over TCP on DNS port 53 with 2 second timeout
                    let addr = std::net::SocketAddr::new(ip, 53);
                    if let Ok(Ok(mut stream)) = tokio::time::timeout(
                        std::time::Duration::from_secs(2),
                        tokio::net::TcpStream::connect(addr)
                    ).await {
                        // Prepare raw AXFR query payload (prefixed by 2-byte length for TCP)
                        let mut q = vec![
                            0x12, 0x34, // ID
                            0x00, 0x00, // Flags: Standard query
                            0x00, 0x01, // QDCOUNT: 1 question
                            0x00, 0x00, // ANCOUNT: 0
                            0x00, 0x00, // NSCOUNT: 0
                            0x00, 0x00, // ARCOUNT: 0
                        ];
                        for part in domain.split('.') {
                            if !part.is_empty() {
                                q.push(part.len() as u8);
                                q.extend_from_slice(part.as_bytes());
                            }
                        }
                        q.push(0x00); // end of name
                        q.push(0x00); q.push(0xfc); // QTYPE: AXFR (252)
                        q.push(0x00); q.push(0x01); // QCLASS: IN (1)

                        let q_len = q.len() as u16;
                        let mut packet = vec![(q_len >> 8) as u8, (q_len & 0xff) as u8];
                        packet.extend(q);

                        use tokio::io::{AsyncWriteExt, AsyncReadExt};
                        if stream.write_all(&packet).await.is_ok() {
                            let mut len_bytes = [0u8; 2];
                            if let Ok(Ok(_)) = tokio::time::timeout(
                                std::time::Duration::from_secs(2),
                                stream.read_exact(&mut len_bytes)
                            ).await {
                                let resp_len = ((len_bytes[0] as usize) << 8) | (len_bytes[1] as usize);
                                if resp_len > 12 {
                                    let mut resp = vec![0u8; resp_len];
                                    if let Ok(Ok(_)) = tokio::time::timeout(
                                        std::time::Duration::from_secs(2),
                                        stream.read_exact(&mut resp)
                                    ).await {
                                        // Simple heuristic: If DNS response returns ANCOUNT > 0
                                        let ancount = ((resp[6] as u16) << 8) | (resp[7] as u16);
                                        if ancount > 0 {
                                            vulnerabilities.push(format!("⚠️ Name Server {} ({}) allows AXFR Zone Transfer", ns_name, ip));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    vulnerabilities
}

// ========== Subdomain Takeover Detection ==========
pub async fn detect_takeovers(client: &Client, subdomains: &[String]) -> Vec<String> {
    let mut vulnerabilities = Vec::new();
    
    let resolver = hickory_resolver::TokioAsyncResolver::tokio(
        hickory_resolver::config::ResolverConfig::default(),
        hickory_resolver::config::ResolverOpts::default(),
    );

    for sub in subdomains {
        // Query CNAME records
        if let Ok(cname_lookup) = resolver.lookup(format!("{}.", sub), hickory_resolver::proto::rr::RecordType::CNAME).await {
            for cname_record in cname_lookup.iter() {
                let cname = cname_record.to_string().to_lowercase();
                
                // Takeover candidates check
                let mut matched_provider = None;
                let mut signatures = Vec::new();

                if cname.contains("github.io") {
                    matched_provider = Some("GitHub Pages");
                    signatures.push("there isn't a github pages site here");
                    signatures.push("404 not found");
                } else if cname.contains("herokuapp.com") {
                    matched_provider = Some("Heroku App");
                    signatures.push("no-such-app");
                    signatures.push("welcome to your new app");
                } else if cname.contains("amazonaws.com") {
                    matched_provider = Some("AWS S3 Bucket");
                    signatures.push("nosuchbucket");
                } else if cname.contains("myshopify.com") {
                    matched_provider = Some("Shopify Store");
                    signatures.push("sorry, this shop is currently unavailable");
                } else if cname.contains("squarespace.com") {
                    matched_provider = Some("Squarespace Site");
                    signatures.push("site not found");
                }

                if let Some(provider) = matched_provider {
                    // CNAME points to a known cloud provider; check if subdomain is active/dangling
                    let url = format!("http://{}", sub);
                    if let Ok(resp) = client.get(&url).send().await {
                        if let Ok(body) = resp.text().await {
                            let lower_body = body.to_lowercase();
                            for sig in signatures {
                                if lower_body.contains(sig) {
                                    vulnerabilities.push(format!("⚠️ Subdomain {} points to dangling {} ({})", sub, provider, cname));
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    vulnerabilities
}