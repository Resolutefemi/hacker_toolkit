//! GUI for Ultimate Hacker Toolkit using egui
//! Provides tabs for each module: Scanner, Stress, Credential Stuffing, Spam, Payload, Report.

use eframe::egui;
use egui::{RichText, Color32, ProgressBar, ScrollArea};
use htool::*;
use mimalloc::MiMalloc;

#[global_allocator]
static  GLOBAL: MiMalloc = MiMalloc;

enum AppMessage {
    ScanProgress(f32),
    ScanFinished(ScanResult),
    StressFinished(Result<u64, String>),
    CredStuffFinished(Vec<LoginResult>),
    SpamFinished(usize),
}

#[derive(PartialEq)]
enum ActiveTab {
    Scanner,
    Stress,
    CredStuff,
    Spam,
    Payload,
    Report,
    CveSearch,
}

struct UltimateApp {
    active_tab: ActiveTab,
    // Scanner
    scan_target: String,
    scan_mode: String,
    scan_rate: u32,
    scan_proxy: String,
    scan_wordlist_path: String,
    scan_timeout: u64,
    scan_result: Option<ScanResult>,
    scan_in_progress: bool,
    scan_progress: f32,
    // Stress
    stress_target: String,
    stress_attack: String,
    stress_threads: usize,
    stress_duration: u64,
    stress_proxy: String,
    stress_result: Option<String>,
    stress_in_progress: bool,
    // Credential Stuffing
    cred_login_url: String,
    cred_user_field: String,
    cred_pass_field: String,
    cred_users_path: String,
    cred_passes_path: String,
    cred_threads: usize,
    cred_proxy_list: String,
    cred_results: Vec<LoginResult>,
    cred_in_progress: bool,
    // Spam
    spam_endpoint: String,
    spam_count: usize,
    spam_threads: usize,
    spam_proxy: String,
    spam_rate: u32,
    spam_result: Option<usize>,
    spam_in_progress: bool,
    // Payload
    payload_type: String,
    payload_platform: String,
    payload_lhost: String,
    payload_lport: u16,
    payload_url: String,
    payload_password: String,
    payload_output: String,
    payload_generated: String,
    // Report
    report_input_path: String,
    report_output_path: String,
    // Logs
    logs: Vec<String>,
    // Channel
    tx: std::sync::mpsc::Sender<AppMessage>,
    rx: std::sync::mpsc::Receiver<AppMessage>,
    // CVE Search tab
    cve_query: String,
    cve_results: Vec<CveEntry>,
}

impl Default for UltimateApp {
    fn default() -> Self {
        let (tx_chan, rx_chan) = std::sync::mpsc::channel();
        Self {
            active_tab: ActiveTab::Scanner,
            scan_target: String::new(),
            scan_mode: "quick".to_string(),
            scan_rate: 10,
            scan_proxy: String::new(),
            scan_wordlist_path: String::new(),
            scan_timeout: 3,
            scan_result: None,
            scan_in_progress: false,
            scan_progress: 0.0,
            stress_target: String::new(),
            stress_attack: "http".to_string(),
            stress_threads: 100,
            stress_duration: 30,
            stress_proxy: String::new(),
            stress_result: None,
            stress_in_progress: false,
            cred_login_url: String::new(),
            cred_user_field: "username".to_string(),
            cred_pass_field: "password".to_string(),
            cred_users_path: String::new(),
            cred_passes_path: String::new(),
            cred_threads: 10,
            cred_proxy_list: String::new(),
            cred_results: Vec::new(),
            cred_in_progress: false,
            spam_endpoint: String::new(),
            spam_count: 100,
            spam_threads: 10,
            spam_proxy: String::new(),
            spam_rate: 20,
            spam_result: None,
            spam_in_progress: false,
            payload_type: "reverse".to_string(),
            payload_platform: "linux".to_string(),
            payload_lhost: String::new(),
            payload_lport: 4444,
            payload_url: String::new(),
            payload_password: String::new(),
            payload_output: String::new(),
            payload_generated: String::new(),
            report_input_path: String::new(),
            report_output_path: String::new(),
            logs: Vec::new(),
            tx: tx_chan,
            rx: rx_chan,
            cve_query: String::new(),
            cve_results: Vec::new(),
        }
    }
}

impl UltimateApp {
    fn add_log(&mut self, msg: String) {
        self.logs.push(format!("[{}] {}", chrono::Local::now().format("%H:%M:%S"), msg));
        if self.logs.len() > 100 {
            self.logs.remove(0);
        }
    }

    fn run_scan(&mut self, ctx: &egui::Context) {
        let target = self.scan_target.clone();
        let mode = self.scan_mode.clone();
        let rate = self.scan_rate;
        let proxy = if self.scan_proxy.is_empty() { None } else { Some(self.scan_proxy.clone()) };
        let wordlist = if !self.scan_wordlist_path.is_empty() {
            load_wordlist(Some(&self.scan_wordlist_path))
        } else {
            load_wordlist(None)
        };
        let timeout = self.scan_timeout;
        let scan_type = if mode == "full" { ScanType::Full } else { ScanType::Quick };
        let config = ScannerConfig {
            scan_type,
            rate_limit_rps: rate,
            proxy,
            wordlist,
            timeout_secs: timeout,
            user_agent: utils::random_user_agent(),
        };
        self.scan_in_progress = true;
        self.scan_progress = 0.0;
        let ctx_clone = ctx.clone();
        let target_clone = target.clone();
        let tx = self.tx.clone();
        tokio::spawn(async move {
            let tx_progress = tx.clone();
            let ctx_progress = ctx_clone.clone();
            let result = run_full_scan(target_clone, config, Some(Box::new(move |progress| {
                let _ = tx_progress.send(AppMessage::ScanProgress(progress));
                ctx_progress.request_repaint();
            }))).await;
            let _ = tx.send(AppMessage::ScanFinished(result));
            ctx_clone.request_repaint();
        });
    }
}

impl eframe::App for UltimateApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Handle messages from background tasks
        while let Ok(msg) = self.rx.try_recv() {
            match msg {
                AppMessage::ScanProgress(p) => {
                    self.scan_progress = p;
                }
                AppMessage::ScanFinished(res) => {
                    self.scan_in_progress = false;
                    self.scan_result = Some(res);
                    self.add_log("Scan completed.".to_string());
                }
                AppMessage::StressFinished(res) => {
                    self.stress_in_progress = false;
                    match res {
                        Ok(sent) => {
                            self.stress_result = Some(format!("Sent {} requests", sent));
                            self.add_log(format!("Stress test completed. Sent {} requests.", sent));
                        }
                        Err(e) => {
                            self.stress_result = Some(format!("Failed: {}", e));
                            self.add_log(format!("Stress test failed: {}", e));
                        }
                    }
                }
                AppMessage::CredStuffFinished(results) => {
                    self.cred_in_progress = false;
                    self.cred_results = results;
                    let successful = self.cred_results.iter().filter(|r| r.success).count();
                    self.add_log(format!("Credential stuffing finished. Successful: {}/{}", successful, self.cred_results.len()));
                }
                AppMessage::SpamFinished(sent) => {
                    self.spam_in_progress = false;
                    self.spam_result = Some(sent);
                    self.add_log(format!("Spam sending completed. Sent {} requests.", sent));
                }
            }
        }

        // Dark mode
        let mut style = (*ctx.style()).clone();
        style.visuals.dark_mode = true;
        ctx.set_style(style);

        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Exit").clicked() {
                        std::process::exit(0);
                    }
                });
                ui.menu_button("Help", |ui| {
                    if ui.button("About").clicked() {
                        // Show about dialog
                    }
                });
            });
        });

        egui::SidePanel::left("sidebar").default_width(180.0).show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("🔧 Modules");
                ui.separator();
                if ui.selectable_label(self.active_tab == ActiveTab::Scanner, "🔍 Scanner").clicked() {
                    self.active_tab = ActiveTab::Scanner;
                }
                if ui.selectable_label(self.active_tab == ActiveTab::Stress, "💥 Stress").clicked() {
                    self.active_tab = ActiveTab::Stress;
                }
                if ui.selectable_label(self.active_tab == ActiveTab::CredStuff, "🔑 Cred Stuff").clicked() {
                    self.active_tab = ActiveTab::CredStuff;
                }
                if ui.selectable_label(self.active_tab == ActiveTab::Spam, "💣 Spam").clicked() {
                    self.active_tab = ActiveTab::Spam;
                }
                if ui.selectable_label(self.active_tab == ActiveTab::Payload, "📡 Payload").clicked() {
                    self.active_tab = ActiveTab::Payload;
                }
                if ui.selectable_label(self.active_tab == ActiveTab::Report, "📊 Report").clicked() {
                    self.active_tab = ActiveTab::Report;
                }
                if ui.selectable_label(self.active_tab == ActiveTab::CveSearch, "🔍 CVE Search").clicked() {
                    self.active_tab = ActiveTab::CveSearch;
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(RichText::new("Ultimate Hacker Toolkit").size(24.0).color(Color32::from_rgb(100, 200, 255)));
            ui.separator();

            match self.active_tab {
                ActiveTab::Scanner => {
                    ui.heading("🔍 Vulnerability Scanner");
                    ui.horizontal(|ui| {
                        ui.label("Target:");
                        ui.text_edit_singleline(&mut self.scan_target);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Mode:");
                        ui.radio_value(&mut self.scan_mode, "quick".to_string(), "⚡ Quick");
                        ui.radio_value(&mut self.scan_mode, "full".to_string(), "🐢 Full");
                    });
                    ui.horizontal(|ui| {
                        ui.label("Rate (RPS):");
                        ui.add(egui::DragValue::new(&mut self.scan_rate).clamp_range(1..=100));
                        ui.label("Proxy:");
                        ui.text_edit_singleline(&mut self.scan_proxy);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Wordlist file:");
                        ui.text_edit_singleline(&mut self.scan_wordlist_path);
                        if ui.button("📂 Browse").clicked() {
                            if let Some(path) = rfd::FileDialog::new().pick_file() {
                                self.scan_wordlist_path = path.display().to_string();
                            }
                        }
                    });
                    ui.horizontal(|ui| {
                        ui.label("Timeout (sec):");
                        ui.add(egui::DragValue::new(&mut self.scan_timeout).clamp_range(1..=30));
                    });
                    if ui.button(RichText::new("▶ Start Scan").size(14.0)).clicked() && !self.scan_in_progress && !self.scan_target.is_empty() {
                        self.run_scan(ctx);
                    }
                    if self.scan_in_progress {
                        ui.add(ProgressBar::new(self.scan_progress).text(format!("{:.0}%", self.scan_progress * 100.0)));
                        ui.spinner();
                    }
                    if let Some(result) = self.scan_result.clone() {
                        ui.separator();
                        ui.heading("Results");
                        ui.label(format!("Open ports: {:?}", result.open_ports));
                        ui.label(format!("SQLi: {} URLs", result.sql_vulnerable.len()));
                        ui.label(format!("XSS: {} URLs", result.xss_vulnerable.len()));
                        ui.label(format!("Technologies: {:?}", result.technologies));
                        ui.label(format!("Subdomain Takeovers: {:?}", result.subdomain_takeovers));
                        ui.label(format!("DNS Zone Transfers: {:?}", result.zone_transfers));
                        ui.add_space(5.0);
                        ui.horizontal(|ui| {
                            if ui.button("📄 Export HTML Report").clicked() {
                                if let Some(path) = rfd::FileDialog::new().add_filter("HTML Files", &["html"]).save_file() {
                                    let path_str = path.display().to_string();
                                    let _ = save_html_report(&result, &path_str);
                                    self.add_log(format!("HTML report exported to {}", path_str));
                                }
                            }
                            if ui.button("💾 Export JSON Report").clicked() {
                                if let Some(path) = rfd::FileDialog::new().add_filter("JSON Files", &["json"]).save_file() {
                                    let path_str = path.display().to_string();
                                    let _ = save_json_report(&result, &path_str);
                                    self.add_log(format!("JSON report exported to {}", path_str));
                                }
                            }
                        });
                    }
                }
                ActiveTab::Stress => {
                    ui.heading("💥 Stress Test (Authorised Only)");
                    ui.horizontal(|ui| {
                        ui.label("Target (http://ip:port):");
                        ui.text_edit_singleline(&mut self.stress_target);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Attack type:");
                        egui::ComboBox::from_id_source("stress_attack_combo").selected_text(&self.stress_attack).show_ui(ui, |ui| {
                            for attack in &["http", "http-random", "slowloris", "udp", "syn", "advanced", "icmp"] {
                                ui.selectable_value(&mut self.stress_attack, attack.to_string(), *attack);
                            }
                        });
                    });
                    ui.horizontal(|ui| {
                        ui.label("Threads:");
                        ui.add(egui::DragValue::new(&mut self.stress_threads).clamp_range(1..=500));
                        ui.label("Duration (sec):");
                        ui.add(egui::DragValue::new(&mut self.stress_duration).clamp_range(1..=3600));
                    });
                    ui.horizontal(|ui| {
                        ui.label("Proxy:");
                        ui.text_edit_singleline(&mut self.stress_proxy);
                    });
                    if ui.button("💥 Launch Attack").clicked() && !self.stress_in_progress {
                        self.stress_in_progress = true;
                        self.stress_result = None;
                        let target = self.stress_target.clone();
                        let attack = self.stress_attack.clone();
                        let threads = self.stress_threads;
                        let duration = self.stress_duration;
                        let proxy = if self.stress_proxy.is_empty() { None } else { Some(self.stress_proxy.clone()) };
                        let tx = self.tx.clone();
                        let ctx_clone = ctx.clone();
                        tokio::spawn(async move {
                            let result = stress::launch_stress_test(&target, &attack, threads, duration, proxy.as_deref()).await;
                            let _ = tx.send(AppMessage::StressFinished(result));
                            ctx_clone.request_repaint();
                        });
                        self.add_log("Stress test started.".to_string());
                    }
                    if let Some(res_str) = &self.stress_result {
                        ui.separator();
                        ui.label(RichText::new(res_str).color(Color32::from_rgb(16, 185, 129)));
                    }
                }
                ActiveTab::CredStuff => {
                    ui.heading("🔑 Credential Stuffing");
                    ui.horizontal(|ui| {
                        ui.label("Login URL:");
                        ui.text_edit_singleline(&mut self.cred_login_url);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Username field:");
                        ui.text_edit_singleline(&mut self.cred_user_field);
                        ui.label("Password field:");
                        ui.text_edit_singleline(&mut self.cred_pass_field);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Users file:");
                        ui.text_edit_singleline(&mut self.cred_users_path);
                        if ui.button("📂").clicked() {
                            if let Some(p) = rfd::FileDialog::new().pick_file() {
                                self.cred_users_path = p.display().to_string();
                            }
                        }
                        ui.label("Passwords file:");
                        ui.text_edit_singleline(&mut self.cred_passes_path);
                        if ui.button("📂").clicked() {
                            if let Some(p) = rfd::FileDialog::new().pick_file() {
                                self.cred_passes_path = p.display().to_string();
                            }
                        }
                    });
                    ui.horizontal(|ui| {
                        ui.label("Threads:");
                        ui.add(egui::DragValue::new(&mut self.cred_threads).clamp_range(1..=50));
                        ui.label("Proxies file:");
                        ui.text_edit_singleline(&mut self.cred_proxy_list);
                    });
                    if ui.button("🚀 Start Stuffing").clicked() && !self.cred_in_progress {
                        self.cred_in_progress = true;
                        self.cred_results.clear();
                        let login_url = self.cred_login_url.clone();
                        let username_field = self.cred_user_field.clone();
                        let password_field = self.cred_pass_field.clone();
                        let usernames = load_wordlist_from_file(&self.cred_users_path);
                        let passwords = load_wordlist_from_file(&self.cred_passes_path);
                        let threads = self.cred_threads;
                        let proxy_list = if self.cred_proxy_list.is_empty() {
                            None
                        } else {
                            Some(load_proxy_list(&self.cred_proxy_list))
                        };
                        let config = CredStuffConfig {
                            login_url,
                            username_field,
                            password_field,
                            extra_fields: vec![],
                            success_indicator: Some("success".to_string()),
                            failure_indicator: None,
                            threads,
                            proxy_list,
                            rate_limit_rps: 5,
                            timeout_secs: 5,
                            user_agent: utils::random_user_agent(),
                        };
                        let tx = self.tx.clone();
                        let ctx_clone = ctx.clone();
                        tokio::spawn(async move {
                            let results = credential_stuffing(&config, usernames, passwords, None).await;
                            let _ = tx.send(AppMessage::CredStuffFinished(results));
                            ctx_clone.request_repaint();
                        });
                        self.add_log("Credential stuffing started.".to_string());
                    }
                    if !self.cred_results.is_empty() {
                        ui.separator();
                        ui.heading("Successful Logins");
                        let successful_list: Vec<_> = self.cred_results.iter().filter(|r| r.success).collect();
                        if successful_list.is_empty() {
                            ui.label("No successful credentials found yet.");
                        } else {
                            for res in successful_list {
                                ui.colored_label(Color32::from_rgb(16, 185, 129), format!("🔑 {}:{}", res.username, res.password));
                            }
                        }
                    }
                }
                ActiveTab::Spam => {
                    ui.heading("💣 Spam & Flood");
                    ui.horizontal(|ui| {
                        ui.label("Endpoint URL:");
                        ui.text_edit_singleline(&mut self.spam_endpoint);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Count:");
                        ui.add(egui::DragValue::new(&mut self.spam_count).clamp_range(1..=10000));
                        ui.label("Threads:");
                        ui.add(egui::DragValue::new(&mut self.spam_threads).clamp_range(1..=100));
                        ui.label("Rate (RPS):");
                        ui.add(egui::DragValue::new(&mut self.spam_rate).clamp_range(1..=100));
                    });
                    ui.horizontal(|ui| {
                        ui.label("Proxy:");
                        ui.text_edit_singleline(&mut self.spam_proxy);
                    });
                    if ui.button("💣 Send Flood").clicked() && !self.spam_in_progress {
                        self.spam_in_progress = true;
                        let endpoint = self.spam_endpoint.clone();
                        let count = self.spam_count;
                        let threads = self.spam_threads;
                        let proxy = if self.spam_proxy.is_empty() { None } else { Some(self.spam_proxy.clone()) };
                        let rate = self.spam_rate;
                        let limiter = create_rate_limiter(rate);
                        let fields = vec![("data", "__RANDOM__")];
                        let tx = self.tx.clone();
                        let ctx_clone = ctx.clone();
                        tokio::spawn(async move {
                            let sent = flood_database(&endpoint, &fields, count, threads, proxy.as_deref(), limiter).await;
                            let _ = tx.send(AppMessage::SpamFinished(sent));
                            ctx_clone.request_repaint();
                        });
                        self.add_log("Database flood started.".to_string());
                    }
                    if let Some(result) = self.spam_result {
                        ui.label(format!("Sent {} requests", result));
                    }
                }
                ActiveTab::Payload => {
                    ui.heading("📡 Payload Generator");
                    ui.horizontal(|ui| {
                        ui.label("Type:");
                        egui::ComboBox::from_id_source("payload_type_combo").selected_text(&self.payload_type).show_ui(ui, |ui| {
                            for t in &["reverse", "bind", "webshell", "downloadexec"] {
                                ui.selectable_value(&mut self.payload_type, t.to_string(), *t);
                            }
                        });
                        ui.label("Platform:");
                        egui::ComboBox::from_id_source("payload_platform_combo").selected_text(&self.payload_platform).show_ui(ui, |ui| {
                            for p in &["linux", "windows", "python", "php", "nodejs", "ruby", "perl"] {
                                ui.selectable_value(&mut self.payload_platform, p.to_string(), *p);
                            }
                        });
                    });
                    if self.payload_type == "reverse" || self.payload_type == "bind" {
                        ui.horizontal(|ui| {
                            ui.label("LHOST/LPORT:");
                            ui.text_edit_singleline(&mut self.payload_lhost);
                            ui.add(egui::DragValue::new(&mut self.payload_lport).clamp_range(1..=65535));
                        });
                    }
                    if self.payload_type == "downloadexec" {
                        ui.horizontal(|ui| {
                            ui.label("URL:");
                            ui.text_edit_singleline(&mut self.payload_url);
                        });
                    }
                    if self.payload_type == "webshell" {
                        ui.horizontal(|ui| {
                            ui.label("Password:");
                            ui.text_edit_singleline(&mut self.payload_password);
                        });
                    }
                    if ui.button("Generate Payload").clicked() {
                        let plat = match self.payload_platform.as_str() {
                            "linux" => Platform::Linux,
                            "windows" => Platform::Windows,
                            "python" => Platform::Python,
                            "php" => Platform::PHP,
                            "nodejs" => Platform::NodeJS,
                            "ruby" => Platform::Ruby,
                            "perl" => Platform::Perl,
                            _ => Platform::Linux,
                        };
                        let gen = match self.payload_type.as_str() {
                            "reverse" => generate_reverse_shell(&self.payload_lhost, self.payload_lport, plat),
                            "bind" => generate_bind_shell(self.payload_lport, plat),
                            "webshell" => {
                                let pass = if self.payload_password.is_empty() { random_webshell_password() } else { self.payload_password.clone() };
                                generate_php_webshell(&pass)
                            }
                            "downloadexec" => generate_download_exec(&self.payload_url, plat),
                            _ => "Unknown".to_string(),
                        };
                        self.payload_generated = gen;
                        self.add_log("Payload generated.".to_string());
                    }
                    ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
                        ui.text_edit_multiline(&mut self.payload_generated);
                    });
                    if !self.payload_output.is_empty() {
                        if ui.button("Save to file").clicked() {
                            if let Some(path) = rfd::FileDialog::new().save_file() {
                                std::fs::write(path, &self.payload_generated).unwrap();
                                self.add_log("Payload saved.".to_string());
                            }
                        }
                    }
                }
                ActiveTab::Report => {
                    ui.heading("📊 Generate Report from JSON");
                    ui.horizontal(|ui| {
                        ui.label("Input JSON file:");
                        ui.text_edit_singleline(&mut self.report_input_path);
                        if ui.button("📂 Browse").clicked() {
                            if let Some(p) = rfd::FileDialog::new().pick_file() {
                                self.report_input_path = p.display().to_string();
                            }
                        }
                    });
                    ui.horizontal(|ui| {
                        ui.label("Output HTML:");
                        ui.text_edit_singleline(&mut self.report_output_path);
                    });
                    if ui.button("Generate Report").clicked() {
                        if let Ok(json) = std::fs::read_to_string(&self.report_input_path) {
                            if let Ok(result) = serde_json::from_str::<ScanResult>(&json) {
                                let out = if self.report_output_path.is_empty() { "report.html".to_string() } else { self.report_output_path.clone() };
                                let _ = save_html_report(&result, &out);
                                self.add_log(format!("Report saved to {}", out));
                            } else {
                                self.add_log("Invalid JSON format".to_string());
                            }
                        } else {
                            self.add_log("Cannot read file".to_string());
                        }
                    }
                }
                ActiveTab::CveSearch => {
                    ui.heading("🔍 Search CVE Offline Database");
                    ui.horizontal(|ui| {
                        ui.label("Search query:");
                        ui.text_edit_singleline(&mut self.cve_query);
                        if ui.button("🔍 Search").clicked() {
                            if !self.cve_query.is_empty() {
                                self.cve_results = search_cves(&self.cve_query);
                                self.add_log(format!("CVE Search returned {} entries.", self.cve_results.len()));
                            } else {
                                self.cve_results.clear();
                            }
                        }
                    });
                    ui.separator();
                    if self.cve_results.is_empty() {
                        ui.label("No results. Enter a query keyword (e.g. 'Apache', 'Log4j', 'Redis') and click Search.");
                    } else {
                        ui.label(format!("Found {} matching entries:", self.cve_results.len()));
                        ScrollArea::vertical().max_height(450.0).show(ui, |ui| {
                            for cve in &self.cve_results {
                                ui.group(|ui| {
                                    ui.horizontal(|ui| {
                                        ui.colored_label(Color32::from_rgb(100, 200, 255), format!("📌 {}", cve.id));
                                        ui.colored_label(Color32::from_rgb(239, 68, 68), format!("CVSS: {}", cve.cvss_score));
                                        ui.label(format!("Year: {}", cve.published_year));
                                    });
                                    ui.label(format!("Affected: {} ({})", cve.product, cve.version_affected));
                                    ui.label(&cve.description);
                                });
                                ui.add_space(5.0);
                            }
                        });
                    }
                }
            }
            ui.separator();
            ui.collapsing("📋 Logs", |ui| {
                ScrollArea::vertical().max_height(150.0).show(ui, |ui| {
                    for log in self.logs.iter().rev() {
                        ui.label(log);
                    }
                });
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let _guard = rt.enter();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([900.0, 600.0])
            .with_title("Ultimate Hacker Toolkit - GUI"),
        ..Default::default()
    };
    eframe::run_native(
        "Ultimate Hacker Toolkit",
        options,
        Box::new(|_cc| Box::new(UltimateApp::default())),
    )
}