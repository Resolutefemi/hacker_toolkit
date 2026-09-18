//! Command-line interface for htool — Ultimate Hacker Toolkit
//! Colored, beginner-friendly output with rich summaries.
//! Supports all modules: scan, stress, credential stuffing, spam, payload, report, CVE search.

use clap::{Parser, Subcommand};
use htool::*;
use std::fs;
use std::io::IsTerminal;
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

// ─── ANSI helpers (auto-disabled when stdout is not a terminal) ───

fn color_enabled() -> bool {
    std::io::stdout().is_terminal()
}

fn paint(code: &str, text: &str) -> String {
    if color_enabled() {
        format!("\x1b[{}m{}\x1b[0m", code, text)
    } else {
        text.to_string()
    }
}

fn c_accent(t: &str) -> String { paint("38;2;0;230;158", t) }   // neon emerald
fn c_cyan(t: &str) -> String { paint("38;2;56;189;248", t) }    // cyan
fn c_red(t: &str) -> String { paint("38;2;248;113;113", t) }    // red
fn c_orange(t: &str) -> String { paint("38;2;251;146;60", t) }  // orange
fn c_yellow(t: &str) -> String { paint("38;2;251;191;36", t) }  // yellow
fn c_purple(t: &str) -> String { paint("38;2;167;139;250", t) } // purple
fn c_dim(t: &str) -> String { paint("38;2;140;152;178", t) }    // dim gray
fn c_bold(t: &str) -> String { paint("1", t) }                  // bold

const BANNER: &str = r#"
   _  _    ___
  | || |  / _ \     ___  ___ _ __ ___   ___  _ __
  | || |_| | | |   / __|/ __| '_ ` _ \ / _ \| '_ \
  |__   _| |_| |   \__ \ (__| | | | | | (_) | | | |
     |_|  \___/    |___/\___|_| |_| |_|\___/|_| |_|
"#;

fn print_banner() {
    if color_enabled() {
        println!("{}", paint("38;2;0;230;158", BANNER));
    } else {
        println!("{}", BANNER);
    }
    println!("  {} v{} — Ultimate Hacker Toolkit", c_bold("htool"), env!("CARGO_PKG_VERSION"));
    println!("  {} by Resolute Femi · authorised security testing only", c_dim("◆"));
    println!();
}

/// Print a key → value summary line
fn kv(key: &str, value: String) {
    println!("  {:<22} {}", key, value);
}

fn print_scan_summary(result: &ScanResult) {
    println!();
    println!("  {}", c_bold("╭─ SCAN RESULTS ─────────────────────────────────────╮"));
    println!("  {}  Target: {}", c_dim("│"), c_cyan(&result.target));
    println!("  {}  Time:   {}", c_dim("│"), c_dim(&result.timestamp));
    println!("  {}", c_dim("├────────────────────────────────────────────────────┤"));
    kv("Open ports:", if result.open_ports.is_empty() { c_dim("none") } else {
        result.open_ports.iter().map(|(p, s)| format!("{}({})", c_accent(&p.to_string()), c_dim(s))).collect::<Vec<_>>().join("  ")
    });
    kv("SQL injection:", if result.sql_vulnerable.is_empty() { c_dim("none found") } else { c_red(&format!("{} vulnerable URL(s)", result.sql_vulnerable.len())) });
    kv("XSS:", if result.xss_vulnerable.is_empty() { c_dim("none found") } else { c_red(&format!("{} vulnerable URL(s)", result.xss_vulnerable.len())) });
    kv("Subdomain takeovers:", if result.subdomain_takeovers.is_empty() { c_dim("none") } else { c_red(&format!("{} critical", result.subdomain_takeovers.len())) });
    kv("DNS zone transfers:", if result.zone_transfers.is_empty() { c_dim("none") } else { c_orange(&format!("{} vulnerable", result.zone_transfers.len())) });
    kv("CVE matches:", if result.cve_matches.is_empty() { c_dim("none") } else { c_yellow(&format!("{} known CVE(s)", result.cve_matches.len())) });
    kv("Discovered paths:", if result.discovered_paths.is_empty() { c_dim("none") } else { c_cyan(&format!("{}", result.discovered_paths.len())) });
    kv("Subdomains:", if result.subdomains.is_empty() { c_dim("none") } else { c_cyan(&format!("{}", result.subdomains.len())) });
    kv("Technologies:", if result.technologies.is_empty() { c_dim("none") } else {
        result.technologies.iter().map(|t| {
            if t.starts_with("[WAF]") { c_orange(t) } else { c_purple(t) }
        }).collect::<Vec<_>>().join("  ")
    });
    kv("SSL/TLS:", c_dim(result.ssl_info.lines().next().unwrap_or("n/a")));
    let sev = result.severity();
    let sev_colored = match sev {
        "CRITICAL" => c_red(sev),
        "HIGH" => c_orange(sev),
        "MEDIUM" => c_yellow(sev),
        "LOW" => c_cyan(sev),
        _ => c_accent(sev),
    };
    kv("Severity:", sev_colored);
    println!("  {}", c_bold("╰────────────────────────────────────────────────────╯"));
    if !result.errors.is_empty() {
        println!();
        println!("  {} {} error(s) during scan:", c_yellow("⚠"), result.errors.len());
        for e in result.errors.iter().take(5) {
            println!("    {} {}", c_dim("·"), c_dim(e));
        }
    }
}

#[derive(Parser)]
#[command(name = "htool")]
#[command(about = "Ultimate Hacker Toolkit — all-in-one authorised security testing", long_about = None)]
#[command(version = env!("CARGO_PKG_VERSION"))]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a vulnerability scan (port, SQLi, XSS, dir brute, subdomains, SSL, headers, CVE)
    Scan {
        /// Target URL or IP address (e.g., https://example.com)
        target: String,
        /// Scan mode: quick or full
        #[arg(short, long, default_value = "quick")]
        mode: String,
        /// Rate limit in requests per second
        #[arg(short, long, default_value = "10")]
        rate: u32,
        /// Proxy URL (e.g., http://127.0.0.1:8080)
        #[arg(short, long)]
        proxy: Option<String>,
        /// Path to custom wordlist file (one entry per line)
        #[arg(short, long)]
        wordlist: Option<String>,
        /// Timeout in seconds for each request
        #[arg(short, long, default_value = "8")]
        timeout: u64,
        /// Output report file path (.html = HTML report, .json = JSON report)
        #[arg(short, long)]
        output: Option<String>,
        /// Also save a JSON file next to the HTML report
        #[arg(long)]
        with_json: bool,
        /// Open the generated report in your browser
        #[arg(long)]
        open: bool,
        /// Print the raw scan JSON to stdout instead of a summary
        #[arg(long)]
        json: bool,
    },
    /// Stress test / load simulation (authorised targets only)
    Stress {
        /// Target URL or IP:port
        target: String,
        /// Attack type: http, http-random, slowloris, udp, syn, advanced, icmp
        #[arg(short, long, default_value = "http")]
        attack: String,
        /// Number of threads / concurrent connections
        #[arg(short, long, default_value = "100")]
        threads: usize,
        /// Duration in seconds
        #[arg(short, long, default_value = "30")]
        duration: u64,
        /// Proxy URL (optional)
        #[arg(short, long)]
        proxy: Option<String>,
    },
    /// Credential stuffing against a login endpoint
    CredStuff {
        /// Login URL (POST endpoint)
        login_url: String,
        /// Username field name
        #[arg(long, default_value = "username")]
        user_field: String,
        /// Password field name
        #[arg(long, default_value = "password")]
        pass_field: String,
        /// Path to username wordlist file
        #[arg(short, long)]
        users: String,
        /// Path to password wordlist file
        #[arg(short, long)]
        passes: String,
        /// Number of concurrent threads
        #[arg(short, long, default_value = "10")]
        threads: usize,
        /// Proxy list file (one per line)
        #[arg(long)]
        proxies: Option<String>,
        /// Success indicator text (e.g., "dashboard")
        #[arg(long, default_value = "dashboard")]
        success_text: String,
        /// Failure indicator text (e.g., "invalid")
        #[arg(long, default_value = "invalid")]
        fail_text: String,
        /// Output file for successful credentials
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Spam / flooding tests
    Spam {
        #[command(subcommand)]
        spam_cmd: SpamCommands,
    },
    /// Generate payloads (reverse/bind shells, web shells)
    Payload {
        /// Payload type: reverse, bind, webshell, downloadexec
        #[arg(short = 't', long = "payload-type", visible_alias = "type")]
        payload_type: String,
        /// Target platform: linux, windows, macos, python, php, nodejs, ruby, perl
        #[arg(short, long)]
        platform: String,
        /// LHOST for reverse shell
        #[arg(short = 'H', long)]
        lhost: Option<String>,
        /// LPORT for reverse/bind shell
        #[arg(short = 'P', long)]
        lport: Option<u16>,
        /// URL for download and execute payload
        #[arg(short, long)]
        url: Option<String>,
        /// Password for web shell (auto-generated if omitted)
        #[arg(long)]
        password: Option<String>,
        /// Output file (optional)
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Generate an HTML report from a saved scan JSON file
    Report {
        /// Path to JSON scan result file
        input: String,
        /// Output HTML file path
        #[arg(short, long)]
        output: Option<String>,
        /// Open the generated report in your browser
        #[arg(long)]
        open: bool,
    },
    /// Search offline CVE database by keyword
    CveSearch {
        /// The search keyword (e.g. Apache, Nginx, PHP)
        query: String,
        /// Show only CVEs with CVSS >= this score
        #[arg(long)]
        min_cvss: Option<f32>,
    },
}

#[derive(Subcommand)]
enum SpamCommands {
    /// Flood a database endpoint with random inserts
    DbFlood {
        /// Target endpoint URL (e.g., http://example.com/insert)
        endpoint: String,
        /// Number of requests to send
        #[arg(short, long, default_value = "100")]
        count: usize,
        /// Number of concurrent threads
        #[arg(short, long, default_value = "10")]
        threads: usize,
        /// Proxy URL (optional)
        #[arg(short, long)]
        proxy: Option<String>,
        /// Rate limit (requests per second)
        #[arg(short, long, default_value = "20")]
        rate: u32,
    },
    /// Email bomber (simulated)
    EmailBomb {
        /// Target email address
        email: String,
        /// Email subject
        subject: String,
        /// Email body template (use __NUM__ for increment)
        body: String,
        /// Number of emails to send
        #[arg(short, long, default_value = "50")]
        count: usize,
        /// Number of concurrent threads
        #[arg(short, long, default_value = "10")]
        threads: usize,
        /// Rate limit
        #[arg(short, long, default_value = "5")]
        rate: u32,
    },
    /// SMS bomber (simulated)
    SmsBomb {
        /// Target phone number
        phone: String,
        /// SMS message template
        message: String,
        /// Number of SMS to send
        #[arg(short, long, default_value = "50")]
        count: usize,
        /// Number of concurrent threads
        #[arg(short, long, default_value = "10")]
        threads: usize,
        /// API key (optional, for real SMS gateway)
        #[arg(short, long)]
        api_key: Option<String>,
    },
    /// Comment spam on a blog or forum
    CommentSpam {
        /// Target URL (where the comment form submits)
        url: String,
        /// Name of the comment field
        #[arg(long, default_value = "comment")]
        comment_field: String,
        /// Name of the name field
        #[arg(long, default_value = "name")]
        name_field: String,
        /// Name of the email field
        #[arg(long, default_value = "email")]
        email_field: String,
        /// Comment template (use __NUM__)
        #[arg(long, default_value = "Great post! #__NUM__")]
        template: String,
        /// Number of comments
        #[arg(short, long, default_value = "100")]
        count: usize,
        /// Threads
        #[arg(short, long, default_value = "10")]
        threads: usize,
        /// Proxy
        #[arg(short, long)]
        proxy: Option<String>,
        /// Rate limit
        #[arg(short, long, default_value = "10")]
        rate: u32,
    },
    /// Registration spam on vulnerable signup pages
    RegSpam {
        /// Signup URL
        url: String,
        /// Number of registrations
        #[arg(short, long, default_value = "50")]
        count: usize,
        /// Threads
        #[arg(short, long, default_value = "5")]
        threads: usize,
        /// Proxy
        #[arg(short, long)]
        proxy: Option<String>,
        /// Rate limit
        #[arg(short, long, default_value = "5")]
        rate: u32,
    },
}

fn open_in_browser(path: &str) {
    println!("  {} Opening {} …", c_cyan("▸"), c_dim(path));
    let _ = open::that(path);
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    print_banner();
    match cli.command {
        Commands::Scan { target, mode, rate, proxy, wordlist, timeout, output, with_json, open, json } => {
            let scan_type = if mode == "full" { ScanType::Full } else { ScanType::Quick };
            let wordlist_vec = load_wordlist(wordlist.as_deref());
            let config = ScannerConfig {
                scan_type,
                rate_limit_rps: rate,
                proxy,
                wordlist: wordlist_vec,
                timeout_secs: timeout,
                user_agent: utils::random_user_agent(),
            };
            println!("  {} {} {} {}",
                c_accent("▸"), c_bold("Scanning"),
                c_cyan(&target),
                c_dim(&format!("({} mode · {} rps · {}s timeout)", mode, rate, timeout)));
            let result = run_full_scan(target, config, None).await;

            if json {
                println!("{}", generate_json_report(&result));
                return;
            }

            print_scan_summary(&result);

            let out_file = output.unwrap_or_else(|| format!("scan_{}.html", result.target.replace('.', "_")));
            let html_path = if out_file.ends_with(".json") {
                // user forced json output; also write html beside it
                let html_alt = out_file.replace(".json", ".html");
                let _ = save_html_report(&result, &html_alt);
                let _ = save_json_report(&result, &out_file);
                println!("\n  {} Reports saved: {} + {}", c_accent("✓"), c_dim(&html_alt), c_dim(&out_file));
                if open { open_in_browser(&html_alt); }
                return;
            } else {
                out_file
            };
            match save_html_report(&result, &html_path) {
                Ok(_) => println!("\n  {} HTML report saved → {}", c_accent("✓"), c_dim(&html_path)),
                Err(e) => println!("  {} {}", c_red("✗"), e),
            }
            if with_json {
                let json_path = if html_path.ends_with(".html") { html_path.replace(".html", ".json") } else { format!("{}.json", html_path) };
                let _ = save_json_report(&result, &json_path);
                println!("  {} JSON report saved → {}", c_accent("✓"), c_dim(&json_path));
            }
            if open { open_in_browser(&html_path); }
        }
        Commands::Stress { target, attack, threads, duration, proxy } => {
            println!("  {} {} {} {}",
                c_orange("▸"), c_bold("Stress test"),
                c_cyan(&target),
                c_dim(&format!("({} · {} threads · {}s)", attack, threads, duration)));
            let count = stress::launch_stress_test(&target, &attack, threads, duration, proxy.as_deref()).await;
            match count {
                Ok(c) => println!("\n  {} Stress test completed. {} requests/connections sent.", c_accent("✓"), c_bold(&c.to_string())),
                Err(e) => println!("\n  {} Stress test failed: {}", c_red("✗"), e),
            }
        }
        Commands::CredStuff { login_url, user_field, pass_field, users, passes, threads, proxies, success_text, fail_text, output } => {
            let usernames = load_wordlist_from_file(&users);
            let passwords = load_wordlist_from_file(&passes);
            let proxy_vec = proxies.map(|p| load_proxy_list(&p));
            let config = CredStuffConfig {
                login_url,
                username_field: user_field,
                password_field: pass_field,
                extra_fields: vec![],
                success_indicator: Some(success_text),
                failure_indicator: Some(fail_text),
                threads,
                proxy_list: proxy_vec,
                rate_limit_rps: 10,
                timeout_secs: 10,
                user_agent: utils::random_user_agent(),
            };
            println!("  {} {} ({} users × {} passwords)",
                c_cyan("▸"), c_bold("Credential stuffing"),
                c_cyan(&usernames.len().to_string()),
                c_cyan(&passwords.len().to_string()));
            let results: Vec<LoginResult> = credential_stuffing(&config, usernames, passwords, None).await;
            let successful: Vec<_> = results.iter().filter(|r| r.success).collect();
            println!();
            if successful.is_empty() {
                println!("  {} Done. {} No valid credentials found.",
                    c_accent("✓"), c_dim(&format!("0/{} attempts succeeded.", results.len())));
            } else {
                println!("  {} Done. {} Found {} valid credential(s):", c_accent("✓"), c_red("⚠"), c_bold(&successful.len().to_string()));
                for res in &successful {
                    println!("    {} {}:{}", c_accent("🔑"), c_cyan(&res.username), c_yellow(&res.password));
                }
            }
            if let Some(out) = output {
                let _ = save_successful_logins(&results, &out);
                println!("\n  {} Saved successful credentials to {}", c_accent("✓"), c_dim(&out));
            }
        }
        Commands::Spam { spam_cmd } => {
            match spam_cmd {
                SpamCommands::DbFlood { endpoint, count, threads, proxy, rate } => {
                    let limiter = create_rate_limiter(rate);
                    let fields = vec![("data", "__RANDOM__"), ("timestamp", "__RANDOM_NUMBER__")];
                    println!("  {} DB flood → {} ({} reqs · {} threads · {} rps)", c_yellow("▸"), c_dim(&endpoint), count, threads, rate);
                    let sent = flood_database(&endpoint, &fields, count, threads, proxy.as_deref(), limiter).await;
                    println!("\n  {} Database flood completed. {} inserts sent.", c_accent("✓"), c_bold(&sent.to_string()));
                }
                SpamCommands::EmailBomb { email, subject, body, count, threads, rate } => {
                    let limiter = create_rate_limiter(rate);
                    println!("  {} Email bomber → {} ({} emails)", c_yellow("▸"), c_dim(&email), count);
                    let sent = email_bomber(&email, &subject, &body, count, threads, None, limiter).await;
                    println!("\n  {} Email bomber completed. {} emails sent (simulated).", c_accent("✓"), c_bold(&sent.to_string()));
                }
                SpamCommands::SmsBomb { phone, message, count, threads, api_key } => {
                    let limiter = create_rate_limiter(5);
                    println!("  {} SMS bomber → {} ({} messages)", c_yellow("▸"), c_dim(&phone), count);
                    let sent = sms_bomber(&phone, &message, count, threads, api_key.as_deref(), limiter).await;
                    println!("\n  {} SMS bomber completed. {} messages sent (simulated).", c_accent("✓"), c_bold(&sent.to_string()));
                }
                SpamCommands::CommentSpam { url, comment_field, name_field, email_field, template, count, threads, proxy, rate } => {
                    let limiter = create_rate_limiter(rate);
                    println!("  {} Comment spam → {} ({} comments)", c_yellow("▸"), c_dim(&url), count);
                    let posted = comment_spam(&url, &comment_field, &name_field, &email_field, &template, count, threads, proxy.as_deref(), limiter).await;
                    println!("\n  {} Comment spam completed. {} comments posted.", c_accent("✓"), c_bold(&posted.to_string()));
                }
                SpamCommands::RegSpam { url, count, threads, proxy, rate } => {
                    let fields = vec![
                        ("username", "__USERNAME__"),
                        ("email", "__EMAIL__"),
                        ("password", "__PASSWORD__"),
                    ];
                    let limiter = create_rate_limiter(rate);
                    println!("  {} Registration spam → {} ({} accounts)", c_yellow("▸"), c_dim(&url), count);
                    let created = registration_spam(&url, &fields, count, threads, proxy.as_deref(), limiter).await;
                    println!("\n  {} Registration spam completed. {} accounts created.", c_accent("✓"), c_bold(&created.to_string()));
                }
            }
        }
        Commands::Payload { payload_type, platform, lhost, lport, url, password, output } => {
            let plat = match platform.as_str() {
                "linux" => Platform::Linux,
                "windows" => Platform::Windows,
                "macos" => Platform::MacOS,
                "python" => Platform::Python,
                "php" => Platform::PHP,
                "nodejs" => Platform::NodeJS,
                "ruby" => Platform::Ruby,
                "perl" => Platform::Perl,
                _ => {
                    println!("  {} Unknown platform '{}'. Use: linux, windows, macos, python, php, nodejs, ruby, perl", c_red("✗"), platform);
                    return;
                }
            };
            let payload_str = match payload_type.as_str() {
                "reverse" => {
                    if let (Some(ip), Some(port)) = (lhost, lport) {
                        generate_reverse_shell(&ip, port, plat)
                    } else {
                        println!("  {} Reverse shell requires --lhost and --lport", c_red("✗"));
                        return;
                    }
                }
                "bind" => {
                    if let Some(port) = lport {
                        generate_bind_shell(port, plat)
                    } else {
                        println!("  {} Bind shell requires --lport", c_red("✗"));
                        return;
                    }
                }
                "webshell" => {
                    if plat == Platform::PHP {
                        let pass = password.unwrap_or_else(random_webshell_password);
                        generate_php_webshell(&pass)
                    } else {
                        println!("  {} Webshell is only supported for the php platform", c_red("✗"));
                        return;
                    }
                }
                "downloadexec" => {
                    if let Some(dl_url) = url {
                        generate_download_exec(&dl_url, plat)
                    } else {
                        println!("  {} Download/exec requires --url", c_red("✗"));
                        return;
                    }
                }
                _ => {
                    println!("  {} Unknown payload type '{}'. Use: reverse, bind, webshell, downloadexec", c_red("✗"), payload_type);
                    return;
                }
            };
            if let Some(out) = output {
                fs::write(&out, &payload_str).expect("Failed to write payload");
                println!("  {} Payload saved to {}", c_accent("✓"), c_dim(&out));
            } else {
                println!("{}", payload_str);
            }
        }
        Commands::Report { input, output, open } => {
            let json_data = match fs::read_to_string(&input) {
                Ok(d) => d,
                Err(e) => {
                    println!("  {} Cannot read '{}': {}", c_red("✗"), input, e);
                    return;
                }
            };
            let result: ScanResult = match serde_json::from_str(&json_data) {
                Ok(r) => r,
                Err(e) => {
                    println!("  {} Invalid scan JSON in '{}': {}", c_red("✗"), input, e);
                    return;
                }
            };
            let out_path = output.unwrap_or_else(|| input.replace(".json", ".html"));
            match save_html_report(&result, &out_path) {
                Ok(_) => {
                    println!("  {} Report generated → {}  ({} findings, severity {})",
                        c_accent("✓"), c_dim(&out_path),
                        c_bold(&result.total_findings().to_string()),
                        c_orange(result.severity()));
                    if open { open_in_browser(&out_path); }
                }
                Err(e) => println!("  {}", c_red(&e)),
            }
        }
        Commands::CveSearch { query, min_cvss } => {
            println!("  {} Searching offline CVE database for {}…", c_cyan("▸"), c_cyan(&query));
            let mut matches = search_cves(&query);
            if let Some(min) = min_cvss {
                matches.retain(|c| c.cvss_score >= min);
            }
            if matches.is_empty() {
                println!("  {} No matching CVEs found.", c_yellow("⚠"));
            } else {
                println!("\n  {} Found {} matching CVE(s):\n", c_accent("✓"), c_bold(&matches.len().to_string()));
                for cve in &matches {
                    let score_colored = if cve.cvss_score >= 9.0 { c_red(&format!("{:.1}", cve.cvss_score)) }
                        else if cve.cvss_score >= 7.0 { c_orange(&format!("{:.1}", cve.cvss_score)) }
                        else { c_yellow(&format!("{:.1}", cve.cvss_score)) };
                    println!("  {} {}  CVSS {}  {}",
                        c_accent("📌"), c_bold(&cve.id), score_colored, c_dim(&cve.published_year.to_string()));
                    println!("     {} {} ({})", c_dim("product:"), c_cyan(&cve.product), c_dim(&cve.version_affected));
                    println!("     {} {}", c_dim("desc:   "), cve.description);
                    println!();
                }
            }
        }
    }
    println!();
}
