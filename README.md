# 🔧 htool

A professional‑grade, all‑in‑one security testing framework and GUI dashboard written in **Rust**.  
Includes modules for vulnerability scanning, stress testing (DoS simulation), credential stuffing, spam/flooding, payload generation, and reporting.

**⚠️ Legal Disclaimer:** This tool is for **authorised security testing and educational purposes only**. Unauthorised use against systems you do not own or have explicit permission to test is illegal. Use at your own risk.

---

## ✨ Features

| Module | Description |
|--------|-------------|
| **Scanner** | Port scanning, SQL injection detection, XSS detection, directory brute‑forcing, subdomain enumeration, SSL/TLS analysis, security headers audit, CVE matching |
| **Advanced Scanning** | DNS Zone Transfer (AXFR) checker, Subdomain Takeover detector (GitHub Pages, S3, Heroku, Shopify, Squarespace), and Technology Fingerprinting (Nginx, Apache, WordPress, React, next.js, Drupal, Nuxt, Vue, PHP, ASP.NET, IIS) |
| **Offline CVE Database** | Browse and query a local CVE database by keywords, CVE ID, product, or description (accessible via CLI and GUI) |
| **Stress** | HTTP flood, Slowloris, UDP flood, SYN flood simulation, advanced HTTP flood with random methods |
| **Credential Stuffing** | Mass login attempts with wordlists, proxy rotation, rate limiting, result logging |
| **Spam** | Database flooding, email bomber, SMS bomber, comment spam, registration spam |
| **Payload** | Reverse shells (Linux, Windows, Python, PHP, Node.js, Ruby, Perl), bind shells, PHP web shells, download & execute |
| **Report** | HTML and JSON report generation, beautiful vulnerability reports with direct GUI export pickers |

---

## 📦 Installation

### Prerequisites
- Rust (1.70+)
- Cargo

### Install from crates.io
Once published, you can install the CLI tool directly from the official registry:
```bash
cargo install htool
```

### Build from source
```bash
git clone https://github.com/Resolutefemi/htool.git
cd htool
cargo build --release
```
* The compiled binary for CLI will be at `./target/release/htool`
* The compiled binary for GUI will be at `./target/release/htool-gui`

---

## 🚀 Usage

### Command Line Interface (CLI)

Get general help:
```bash
./target/release/htool --help
```

#### 1. Vulnerability Scanner (`scan`)
Scan a target URL or host (includes port scanning, SQLi, XSS, directory brute forcing, subdomain discovery, SSL analysis, security header audits, and CVE matching).
```bash
# Quick scan with default options
./target/release/htool scan example.com

# Full scan with a custom proxy and rate limits, saving HTML report
./target/release/htool scan example.com --mode full --rate 15 --proxy http://127.0.0.1:8080 --output report.html
```

#### 2. Offline CVE Lookup (`cve-search`)
Query the local database of Common Vulnerabilities and Exposures by keywords.
```bash
./target/release/htool cve-search Apache
```

#### 3. Payload Generation (`payload`)
Generate reverse shells, bind shells, web shells, and download/execution payloads across multiple languages.
```bash
# Generate a reverse shell payload
./target/release/htool payload --payload-type reverse --platform linux --lhost 10.10.10.10 --lport 4444

# Generate a PHP web shell and save to a file
./target/release/htool payload --payload-type webshell --platform php --password secretpass --output shell.php
```

#### 4. Stress Testing (`stress`)
Simulate network load on targets.
```bash
# HTTP flood with 50 threads for 60 seconds
./target/release/htool stress http://192.168.1.1 --attack http --threads 50 --duration 60
```

#### 5. Credential Stuffing (`cred-stuff`)
Perform mass login tests against an authentication endpoint.
```bash
./target/release/htool cred-stuff http://example.com/login --users users.txt --passes passwords.txt --threads 10 --success-text "dashboard"
```

#### 6. Spam & Flooding (`spam`)
Test rate limiting on databases, forums, forms, and services.
```bash
# Flood a database API endpoint with random data
./target/release/htool spam db-flood http://example.com/api/insert --count 500 --threads 20
```

#### 7. Report Generation (`report`)
Create HTML reports from raw scan JSON files.
```bash
./target/release/htool report scan_results.json --output report.html
```

---

### Graphical User Interface (GUI)

Launch the interactive dashboard with real-time logs, live async progress bars, and file export options:
```bash
./target/release/htool-gui
```

* **Desktop Icon:** The Windows executable is pre-packaged with a custom high-tech glowing cybersecurity shield icon (`assets/icon.ico`).