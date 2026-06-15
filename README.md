# 🔥 Ultimate Hacker Toolkit

A professional‑grade, all‑in‑one security testing framework written in **Rust**.  
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

### Build from source
```bash
git clone https://github.com/Resolutefemi/hacker_toolkit.git
cd hacker_toolkit
cargo build --release
```

## 🚀 Usage

### Command Line Interface (CLI)

```bash
# Run a vulnerability scan (Includes advanced checks: Tech stack, AXFR, Takeovers)
./target/release/htool scan example.com --mode full --output report.html

# Search the offline CVE database
./target/release/htool cve-search Apache

# Generate a shell payload
./target/release/htool payload --payload-type reverse --platform linux --lhost 10.10.10.10 --lport 4444
```

### Graphical User Interface (GUI)

Launch the reactive user interface featuring live async progress updates, logs, and export managers:
```bash
./target/release/htool-gui
```