# 🔧 htool

A professional-grade, all-in-one security testing framework with a **modern desktop GUI** and **CLI**, written in **Rust**.
Includes modules for vulnerability scanning, scheduled scanning, stress testing (DoS simulation), credential stuffing, spam/flooding, payload generation, and HTML/JSON/PDF reporting.

**⚠️ Legal Disclaimer:** This tool is for **authorised security testing and educational purposes only**. Unauthorised use against systems you do not own or have explicit permission to test is illegal. Use at your own risk.

---

## ✨ What's new in v3.2

- ⏱ **Scan Scheduler** — schedule scans *every N minutes* or *daily at HH:MM*. The GUI gets a full Scheduler tab (countdowns, pause/enable, run-now, last-status history); the CLI gets `htool schedule add/list/toggle/remove` plus a `schedule run` daemon. Scheduled scans automatically write **HTML + JSON + PDF** reports to their reports folder. Everything persists in `~/.htool/schedules.json`
- 📕 **PDF reports** — beautifully typeset A4 PDF reports with a branded header banner, severity stat grid, colored findings sections and remediation recommendations. Zero dependencies (hand-rolled PDF engine, core Helvetica fonts). Export from the GUI (📕 Export PDF), or CLI: `htool report scan.json --pdf`, `htool scan target --output report.pdf`, `--with-pdf`
- ☀️ **Dark / Light mode toggle** — full dual-palette redesign of the GUI. One click in the top bar switches the entire dashboard (cards, chips, code blocks, progress bars); your choice is remembered in `~/.htool/ui.json`
- 🌍 **Website deploys itself now** — the site builds & publishes to **GitHub Pages on every push** with zero secrets/tokens required (see [Website deployment](#-website-deployment)) — plus the site got its own light/dark toggle

## ✨ What was new in v3.1

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
| **Report** | Modern **HTML + JSON + PDF** reports, in-app rendered report viewer, open-in-browser export |
| **Scheduler** | Recurring scans (interval or daily) with persistent schedules, automatic HTML+JSON+PDF report writing, run history |

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
# HTML report (default)
htool report scan_results.json --output report.html --open

# PDF report
htool report scan_results.json --pdf

# Scan and save all three formats at once
htool scan example.com --output report.html --with-json --with-pdf
htool scan example.com --output report.pdf   # PDF only
```

#### 8. Scan Scheduler (`schedule`)
```bash
# Scan every hour, reports land in ~/.htool/reports
htool schedule add https://example.com --name "hourly check" --every 3600

# Scan every day at 09:00 local time
htool schedule add https://example.com --daily 09:00 --mode full

# Manage
htool schedule list
htool schedule toggle sc-5c8961          # pause / resume
htool schedule remove sc-5c8961

# Start the scheduler daemon — fires due scans automatically (Ctrl+C to stop)
htool schedule run
```
Schedules persist in `~/.htool/schedules.json`; each run writes `reports/<name>_<timestamp>.html/.json/.pdf` and records the last status. The GUI has the same scheduler with live countdowns — no terminal needed.

---

### Graphical User Interface (GUI)

Launch the redesigned interactive dashboard:
```bash
htool-gui
```

**v3.2 GUI highlights**
- **Command Center** — module launcher cards + last-scan severity overview
- **Scan Scheduler tab** — build schedules (every N min / daily at HH:MM), see next-run countdowns, pause/enable, run-now, last-run status per schedule
- **Scanner** — config card, live phase progress bar, then a full *rendered report preview* with stat grid, severity badge, port/tech chips and colored vulnerability lists — plus **Export HTML / PDF / JSON** buttons
- **Report Viewer** — load any scan JSON: read the rendered report in-app (or switch to the tinted JSON view), export HTML/**PDF**/JSON, or open in browser
- **☀️/☾ Theme toggle** — top-right button switches the entire dashboard between dark and light mode; preference is saved
- **Payload, Stress, Cred Stuffing, Spam & CVE modules** — all restyled with cards, badges and one-click copy/save actions
- **Activity log** — always-visible, timestamped log strip

* **Desktop Icon:** The Windows executable ships with a custom high-tech glowing cybersecurity shield icon (`assets/icon.ico`).

---

## 🌍 Website deployment

The website (`website/`) is a static Next.js export. It deploys **automatically to GitHub Pages** on every push to `main` that touches `website/**` — no API tokens, no secrets, no manual steps:

- Site URL: `https://resolutefemi.github.io/hacker_toolkit/`
- Workflow: [.github/workflows/website.yml](.github/workflows/website.yml) (uses the built-in `GITHUB_TOKEN`)
- First deploy: after the workflow succeeds once, open **Settings → Pages** and confirm the source is *GitHub Actions* (GitHub usually provisions this automatically)

<details>
<summary><b>Prefer Cloudflare Pages (htoolapp.pages.dev)? No token needed either.</b></summary>

The old workflow used a `CLOUDFLARE_API_TOKEN` secret to push the site via the API. That token is **not compulsory** — Cloudflare can build the site itself:

1. Go to the [Cloudflare dashboard → Workers & Pages](https://dash.cloudflare.com/) → **Create → Pages → Connect to Git**
2. Select the `hacker_toolkit` repo
3. Build settings:
   - Build command: `cd website && npm ci && npm run build`
   - Output directory: `website/out`
4. Save — Cloudflare now rebuilds the site on every push (leave `NEXT_PUBLIC_BASE_PATH` unset so assets resolve at the domain root)

If an old direct-upload `htoolapp` project exists, delete or rename it first so the git-connected project can reuse the `htoolapp.pages.dev` subdomain.
</details>
