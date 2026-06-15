# 🔥 Ultimate Hacker Toolkit

A professional‑grade, all‑in‑one security testing framework written in **Rust**.  
Includes modules for vulnerability scanning, stress testing (DoS simulation), credential stuffing, spam/flooding, payload generation, and reporting.

**⚠️ Legal Disclaimer:** This tool is for **authorised security testing and educational purposes only**. Unauthorised use against systems you do not own or have explicit permission to test is illegal. Use at your own risk.

---

## ✨ Features

| Module | Description |
|--------|-------------|
| **Scanner** | Port scanning, SQL injection detection, XSS detection, directory brute‑forcing, subdomain enumeration, SSL/TLS analysis, security headers audit, CVE matching |
| **Stress** | HTTP flood, Slowloris, UDP flood, SYN flood simulation, advanced HTTP flood with random methods |
| **Credential Stuffing** | Mass login attempts with wordlists, proxy rotation, rate limiting, result logging |
| **Spam** | Database flooding, email bomber, SMS bomber, comment spam, registration spam |
| **Payload** | Reverse shells (Linux, Windows, Python, PHP, Node.js, Ruby, Perl), bind shells, PHP web shells, download & execute |
| **Report** | HTML and JSON report generation, beautiful vulnerability reports |

---

## 📦 Installation

### Prerequisites
- Rust (1.70+)
- Cargo

### Build from source
```bash
git clone https://github.com/yourusername/ultimate_hacker_toolkit.git
cd ultimate_hacker_toolkit
cargo build --release