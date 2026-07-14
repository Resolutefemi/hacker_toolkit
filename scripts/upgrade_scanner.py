#!/usr/bin/env python3
"""Upgrade scanner.rs with massive technology fingerprinting and WAF detection."""
import sys

with open('src/scanner.rs', 'r', encoding='utf-8', errors='ignore') as f:
    content = f.read()

# Find the fingerprint_technologies function
start = content.find('pub async fn fingerprint_technologies')
if start == -1:
    print('ERROR: fingerprint_technologies not found')
    sys.exit(1)

# Find the end (next section)
end = content.find('\n// ==========', start + 10)
if end == -1:
    print('ERROR: could not find end of function')
    sys.exit(1)

new_func = r'''pub async fn fingerprint_technologies(client: &Client, base_url: &Url, headers_str: &str) -> Vec<String> {
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
                if let Some(s) = body.find("?ver=") {
                    let e = (s+5 .. std::cmp::min(s+20, body.len())).find(|p| !body.as_bytes()[p].is_ascii_alphanumeric() && body.as_bytes()[p] != b'.').unwrap_or(15);
                    if e > 4 { techs.push(format!("WordPress v{}", &body[s+5..s+5+e])); }
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
'''

content = content[:start] + new_func + content[end:]

# Also add waf_results to the orchestrator - find the run_full_scan function
# Find where technologies are fingerprinted and add WAF detection
waf_call = '''
    // 9b. WAF Detection
    result.cve_matches.extend(cve::check_cves(&client, &base_url).await);
    let waf_results = detect_waf(&client, &base_url).await;
    if !waf_results.is_empty() {
        result.cve_matches.push("---- WAF DETECTION ----".to_string());
        result.cve_matches.extend(waf_results);
    }
'''
# Insert after SSL check line
ssl_check = 'result.ssl_info = check_ssl(&host).await;'
content = content.replace(ssl_check, ssl_check + waf_call)

with open('src/scanner.rs', 'w', encoding='utf-8') as f:
    f.write(content)

print('SUCCESS: Scanner.rs upgraded!')
print(f'New fingerprint function: {len(new_func)} chars')
