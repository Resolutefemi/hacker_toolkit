# 🔧 htool

A professional-grade, all-in-one security testing framework with a **modern desktop GUI** and **CLI**, written in **Rust**.
Includes modules for vulnerability scanning, stress testing (DoS simulation), credential stuffing, spam/flooding, payload generation, and reporting.

**⚠️ Legal Disclaimer:** This tool is for **authorised security testing and educational purposes only**. Unauthorised use against systems you do not own or have explicit permission to test is illegal. Use at your own risk.

---

## ✨ What's new in v3.1

- 🎨 **Fully redesigned GUI** — deep-navy cyber theme with neon-emerald accents, rounded cards, stat grids, severity badges and hover states across every module
- 📊 **In-app Report Viewer** — open any scan JSON and read the **fully rendered report inside the app** (stat cards, chips, code blocks — not raw source), plus a syntax-tinted JSON view
- 🌐 **Open reports in your browser** — one click generates & opens the modern HTML report
- 🖥️ **New Command Center dashboard** — module launcher grid + severity overview of your latest scan
- 🌈 **Modern CLI** — ASCII banner, true-color output (auto-disabled when piped), rich scan summary tables, `--json` output, `--open` and `--with-json` flags
- 📄 **Redesigned HTML reports** — severity banner, summary stat grid, accent cards, technology/WAF chips
- 🐛 **Fixed broken features** — `payload` subcommand no longer panics (duplicate `-p` / `-l` flags), WAF detection results are no longer overwritten by tech fingerprinting, CI workflow YAML repaired, missing `assets/wordlist.txt` restored
- ⚡ **New scanner severity engine** — every scan gets a CRITICAL / HIGH / MEDIUM / LOW / INFO rating

## ✨ Features

| Module | Description |
|--------|-------------|
| **Scanner** | Port scanning, SQL injection detection, XSS detection, directory brute-forcing, subdomain enumeration, SSL/TLS analysis, security headers audit, CVE matching |
| **Advanced Scanning** | DNS Zone Transfer (AXFR) checker, Subdomain Takeover detector (GitHub Pages, S3, Heroku, Shopify, Squarespace), WAF detection (Cloudflare, Sucuri, ModSecurity, AWS, Imperva, F5…), Technology Fingerprinting (100+ signatures) |
| **Offline CVE Database** | Browse and query a local CVE database by keywords, CVE ID, product, or description (CLI and GUI) |
| **Stress** | HTTP flood, Slowloris, UDP flood, SYN flood simulation, advanced HTTP flood with random methods |
| **Credential Stuffing** | Mass login attempts with wordlists, proxy rotation, rate limiting, result logging |
| **Spam** | Database flooding, comment spam, registration spam — rate-limit testing |
| **Payload** | Reverse shells (Linux, Windows, macOS, Python, PHP, Node.js, Ruby, Perl), bind shells, PHP web shells, download & execute |
| **Report** | Modern HTML + JSON reports, in-app rendered report viewer, open-in-browser export |

---

## 📦 Installation

### Download prebuilt binaries
Grab the latest release for your platform from the [Releases page](https://github.com/Resolutefemi/hacker_toolkit/releases/latest):
- **Windows**: `htool-x86_64-pc-windows-msvc.zip` → contains `htool-gui.exe` (desktop dashboard) + `htool.exe` (CLI)
- **Linux**: `htool-x86_64-unknown-linux-gnu.tar.gz`
- **macOS**: `htool-x86_64-apple-darwin.tar.gz`

### Install from crates.io
```bash
cargo install htool
```

### Build from source
```bash
git clone https://github.com/Resolutefemi/hacker_toolkit.git
cd hacker_toolkit
cargo build --release
```
* The compiled CLI binary will be at `./target/release/htool`
* The compiled GUI binary will be at `./target/release/htool-gui`

---

## 🚀 Usage

### Command Line Interface (CLI)

```bash
htool --help
```

#### 1. Vulnerability Scanner (`scan`)
```bash
# Quick scan with default options
htool scan example.com

# Full scan (ports 1-1024) with custom rate, saving HTML report and opening it in your browser
htool scan example.com --mode full --rate 15 --output report.html --open

# Print machine-readable JSON instead of the colored summary
htool scan example.com --json
```
The scan ends with a colored results table: open ports, SQLi/XSS hits, takeovers, zone transfers, CVE matches, WAF/tech stack and an overall severity rating.

#### 2. Offline CVE Lookup (`cve-search`)
```bash
htool cve-search Apache
htool cve-search log4j --min-cvss 9.0
```

#### 3. Payload Generation (`payload`)
```bash
# Generate a reverse shell payload
htool payload --payload-type reverse --platform linux --lhost 10.10.10.10 --lport 4444

# Short flags: -t type, -p platform, -H lhost, -P lport
htool payload -t webshell -p php --password secretpass --output shell.php
```

#### 4. Stress Testing (`stress`)
```bash
htool stress http://192.168.1.1 --attack http --threads 50 --duration 60
```

#### 5. Credential Stuffing (`cred-stuff`)
```bash
htool cred-stuff http://example.com/login --users users.txt --passes passwords.txt --threads 10 --success-text "dashboard"
```

#### 6. Spam & Flooding (`spam`)
```bash
htool spam db-flood http://example.com/api/insert --count 500 --threads 20
```

#### 7. Report Generation (`report`)
```bash
htool report scan_results.json --output report.html --open
```

---

### Graphical User Interface (GUI)

Launch the redesigned interactive dashboard:
```bash
htool-gui
```

**v3.1 GUI highlights**
- **Command Center** — module launcher cards + last-scan severity overview
- **Scanner** — config card, live phase progress bar, then a full *rendered report preview* with stat grid, severity badge, port/tech chips and colored vulnerability lists
- **Report Viewer** — load any scan JSON: read the rendered report in-app (or switch to the tinted JSON view), export HTML/JSON, or open in browser
- **Payload, Stress, Cred Stuffing, Spam & CVE modules** — all restyled with cards, badges and one-click copy/save actions
- **Activity log** — always-visible, timestamped log strip

* **Desktop Icon:** The Windows executable ships with a custom high-tech glowing cybersecurity shield icon (`assets/icon.ico`).
