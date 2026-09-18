import Head from 'next/head'
import { useState, useEffect, useRef } from 'react'
import {
  IconScanner, IconStress, IconCredential, IconSpam,
  IconPayload, IconReport, IconCve, IconHtmlViewer, IconWaf,
  OsWindows, OsMacos, OsLinux, HIcon
} from '../components/icons'

const RELEASE_BASE = 'https://github.com/Resolutefemi/hacker_toolkit/releases/latest/download'
const RELEASES_PAGE = 'https://github.com/Resolutefemi/hacker_toolkit/releases/latest'

const ASSETS = {
  windows: `${RELEASE_BASE}/htool-x86_64-pc-windows-msvc.zip`,
  linux: `${RELEASE_BASE}/htool-x86_64-unknown-linux-gnu.tar.gz`,
  macos: `${RELEASE_BASE}/htool-x86_64-apple-darwin.tar.gz`,
}

// ─── Hooks ─────────────────────────────────────────────────

function useOs() {
  const [os, setOs] = useState('windows')
  useEffect(() => {
    const ua = navigator.userAgent.toLowerCase()
    if (ua.includes('win')) setOs('windows')
    else if (ua.includes('mac')) setOs('macos')
    else if (ua.includes('linux') || ua.includes('x11')) setOs('linux')
    else setOs('windows')
  }, [])
  return os
}

function useReveal() {
  useEffect(() => {
    const els = document.querySelectorAll('.reveal')
    const io = new IntersectionObserver((entries) => {
      entries.forEach(e => {
        if (e.isIntersecting) {
          e.target.classList.add('visible')
          io.unobserve(e.target)
        }
      })
    }, { threshold: 0.12 })
    els.forEach(el => io.observe(el))
    return () => io.disconnect()
  }, [])
}

function useLatestRelease() {
  const [rel, setRel] = useState({ tag: 'v3.2.0', assets: 3, loading: true })
  useEffect(() => {
    fetch('https://api.github.com/repos/Resolutefemi/hacker_toolkit/releases/latest')
      .then(r => r.ok ? r.json() : Promise.reject())
      .then(d => setRel({ tag: d.tag_name, assets: d.assets?.length || 3, loading: false }))
      .catch(() => setRel({ tag: 'v3.2.0', assets: 3, loading: false }))
  }, [])
  return rel
}

// ─── Typing terminal ───────────────────────────────────────

const TERMINAL_LINES = [
  { pre: '$ ', cmd: true, parts: [{ t: 'htool scan', c: 'text-matrix-500 font-bold' }, { t: ' https://target.com', c: 'text-cyber-cyan' }] },
  { dim: true, text: '▸ multi-threaded async scan started (10 rps)' },
  { text: '✓ Open ports:', accent: true, value: ' 22·SSH  80·HTTP  443·HTTPS  3306·MySQL' },
  { text: '⚠ SQL injection:', danger: true, value: ' /item?id=1  /product?cat=2' },
  { text: '⚠ XSS:', danger: true, value: ' /search?q=' },
  { text: '◈ WAF:', orange: true, value: ' Cloudflare WAF detected' },
  { text: '🛠 Tech:', purple: true, value: ' Nginx · PHP 8.1 · WordPress v6.4' },
  { dim: true, text: '▸ severity: HIGH · 12.4s total' },
  { text: '✓ Report saved →', accent: true, value: ' htool_report.html' },
]

function Terminal() {
  const [lines, setLines] = useState(0)
  useEffect(() => {
    if (lines >= TERMINAL_LINES.length) {
      const restart = setTimeout(() => setLines(0), 5200)
      return () => clearTimeout(restart)
    }
    const t = setTimeout(() => setLines(l => l + 1), lines === 0 ? 700 : 520)
    return () => clearTimeout(t)
  }, [lines])

  return (
    <div className="terminal-window w-full max-w-xl text-left">
      <div className="terminal-header">
        <div className="terminal-dot" style={{ background: '#f87171' }} />
        <div className="terminal-dot" style={{ background: '#fbbf24' }} />
        <div className="terminal-dot" style={{ background: '#34d399' }} />
        <span className="text-cyber-faint text-xs ml-2 font-mono">htool — zsh</span>
      </div>
      <div className="terminal-content p-4 sm:p-5 overflow-x-auto text-[12.5px] sm:text-[13px] min-h-[218px]">
        {TERMINAL_LINES.slice(0, lines).map((l, i) => (
          <div key={i} className="whitespace-nowrap">
            {l.cmd ? (
              <>
                <span className="text-matrix-500">{l.pre}</span>
                {l.parts.map((p, j) => <span key={j} className={p.c}>{p.t}</span>)}
              </>
            ) : (
              <span className={l.dim ? 'text-cyber-faint' : 'text-cyber-dim'}>
                {l.text && <span className={l.accent ? 'text-matrix-500' : l.danger ? 'text-red-400' : l.orange ? 'text-cyber-orange' : l.purple ? 'text-cyber-purple' : ''}>{l.text}</span>}
                {l.value && <span>{l.value}</span>}
              </span>
            )}
          </div>
        ))}
        {lines >= TERMINAL_LINES.length && (
          <span className="text-matrix-500">$ <span className="inline-block w-2 h-4 bg-matrix-500 align-middle animate-blink" /></span>
        )}
        {lines < TERMINAL_LINES.length && (
          <span className="text-matrix-500">{TERMINAL_LINES[lines].cmd ? '$ ' : ''}<span className="inline-block w-2 h-4 bg-matrix-500/70 align-middle animate-blink" /></span>
        )}
      </div>
    </div>
  )
}

// ─── Theme toggle ──────────────────────────────────────────

function ThemeToggle() {
  const [light, setLight] = useState(false)
  useEffect(() => {
    setLight(document.documentElement.classList.contains('light'))
  }, [])
  const flip = () => {
    const next = !light
    setLight(next)
    document.documentElement.classList.toggle('light', next)
    try { localStorage.setItem('htool-theme', next ? 'light' : 'dark') } catch (e) {}
  }
  return (
    <button onClick={flip} className="theme-toggle" aria-label={light ? 'Switch to dark mode' : 'Switch to light mode'} title={light ? 'Switch to dark mode' : 'Switch to light mode'}>
      {light ? '☾' : '☀'}
    </button>
  )
}

// ─── Header ────────────────────────────────────────────────

function Header() {
  const [menuOpen, setMenuOpen] = useState(false)
  const [scrolled, setScrolled] = useState(false)
  useEffect(() => {
    const fn = () => setScrolled(window.scrollY > 24)
    window.addEventListener('scroll', fn)
    return () => window.removeEventListener('scroll', fn)
  }, [])

  const links = [['#features', 'Features'], ['#app', 'App UI'], ['#download', 'Download'], ['#faq', 'FAQ'], ['#author', 'Author']]

  return (
    <header className={`fixed top-0 left-0 right-0 z-50 transition-all duration-300 ${scrolled ? 'glass border-b border-white/5' : 'bg-transparent'}`}>
      <div className="max-w-6xl mx-auto px-4 h-16 flex items-center justify-between">
        <a href="#top" className="flex items-center gap-2.5 group">
          <div className="w-8 h-8 rounded-lg bg-matrix-500/15 border border-matrix-500/40 flex items-center justify-center group-hover:bg-matrix-500/25 transition-colors">
            <HIcon className="w-5 h-5" />
          </div>
          <span className="text-white text-xl font-bold font-mono tracking-tight">htool</span>
          <span className="text-[10px] font-mono text-matrix-500 bg-matrix-500/10 border border-matrix-500/30 px-1.5 py-0.5 rounded-full">v3.2</span>
        </a>

        <nav className="hidden md:flex items-center gap-7">
          {links.map(([href, label]) => (
            <a key={href} href={href} className="text-cyber-dim hover:text-matrix-500 transition-colors text-[13.5px] font-medium">{label}</a>
          ))}
          <ThemeToggle />
          <a href={ASSETS.windows} className="bg-matrix-500 text-black px-4 py-2 rounded-lg font-semibold text-[13.5px] hover:bg-matrix-400 hover:shadow-glow-accent transition-all btn-flash flex items-center gap-1.5">
            <OsWindows className="w-3.5 h-3.5" /> Download
          </a>
        </nav>

        <button onClick={() => setMenuOpen(!menuOpen)} className="md:hidden flex flex-col gap-1.5 p-2" aria-label="Toggle menu">
          <span className={`block w-6 h-0.5 bg-matrix-500 transition-all ${menuOpen ? 'rotate-45 translate-y-2' : ''}`} />
          <span className={`block w-6 h-0.5 bg-matrix-500 transition-all ${menuOpen ? 'opacity-0' : ''}`} />
          <span className={`block w-6 h-0.5 bg-matrix-500 transition-all ${menuOpen ? '-rotate-45 -translate-y-2' : ''}`} />
        </button>
      </div>

      <div className={`md:hidden fixed inset-0 top-16 bg-cyber-dark/97 backdrop-blur-xl transition-all duration-300 ${menuOpen ? 'opacity-100 pointer-events-auto' : 'opacity-0 pointer-events-none'}`}>
        <div className="flex flex-col items-center justify-center h-full gap-7">
          {links.map(([href, label]) => (
            <a key={href} href={href} onClick={() => setMenuOpen(false)} className="text-2xl text-cyber-dim hover:text-matrix-500 font-mono">[ {label} ]</a>
          ))}
          <ThemeToggle />
        </div>
      </div>
    </header>
  )
}

// ─── Hero ──────────────────────────────────────────────────

function Hero() {
  const os = useOs()
  const rel = useLatestRelease()
  const stats = [
    ['9', 'modules'], ['50+', 'CVE entries'], ['100+', 'tech fingerprints'], ['7', 'payload platforms'],
  ]

  return (
    <section id="top" className="hero-glow relative pt-32 pb-20 md:pt-40 md:pb-28 overflow-hidden">
      <div className="absolute inset-0 bg-grid" />
      <div className="max-w-6xl mx-auto px-4 sm:px-6 relative z-10 grid lg:grid-cols-[1.05fr_0.95fr] gap-12 items-center">
        <div>
          <div className="inline-flex items-center gap-2 bg-matrix-500/10 border border-matrix-500/30 text-matrix-500 text-xs font-mono px-3.5 py-1.5 rounded-full mb-7">
            <span className="w-1.5 h-1.5 rounded-full bg-matrix-500 animate-pulse" />
            v3.2.0 — PDF reports · scan scheduler · light mode
          </div>
          <h1 className="font-display text-[2.6rem] leading-[1.06] sm:text-6xl xl:text-[4.4rem] font-bold text-white mb-6">
            The hacker toolkit,
            <br />
            <span className="text-gradient whitespace-nowrap">re-engineered.</span>
          </h1>
          <p className="text-cyber-dim text-base sm:text-lg max-w-xl mb-8 leading-relaxed">
            <span className="text-matrix-500 font-semibold">htool</span> packs vulnerability scanning, stress testing,
            credential auditing and payload generation into one blazing-fast Rust binary — with a
            brand-new modern dashboard and reports you can read right inside the app.
          </p>
          <div className="flex flex-col sm:flex-row gap-3.5 mb-9">
            <a href={ASSETS[os]} className="bg-matrix-500 text-black px-7 py-4 rounded-xl font-bold text-base hover:bg-matrix-400 hover:shadow-glow-accent transition-all btn-flash flex items-center justify-center gap-2.5 shadow-card">
              <OsWindows className="w-5 h-5" />
              <span className="flex flex-col items-start leading-tight">
                <span>Download for {os === 'windows' ? 'Windows' : os === 'macos' ? 'macOS' : 'Linux'}</span>
                <span className="text-[10px] font-mono opacity-70">{os === 'windows' ? 'htool-x86_64.exe · GUI + CLI' : 'tar.gz · GUI + CLI'}</span>
              </span>
            </a>
            <a href="#features" className="border border-white/15 text-cyber-bright px-7 py-4 rounded-xl font-semibold hover:border-matrix-500/60 hover:text-matrix-500 transition-all flex items-center justify-center gap-2">
              Explore features <span className="text-matrix-500">→</span>
            </a>
          </div>
          <div className="grid grid-cols-4 gap-3 max-w-md">
            {stats.map(([n, l]) => (
              <div key={l} className="text-center sm:text-left">
                <div className="text-xl sm:text-2xl font-bold font-mono text-matrix-500">{n}</div>
                <div className="text-[10.5px] uppercase tracking-wider text-cyber-faint">{l}</div>
              </div>
            ))}
          </div>
        </div>

        <div className="flex flex-col items-center gap-4 animate-float-slow">
          <Terminal />
          <div className="flex items-center gap-2 text-cyber-faint text-xs font-mono">
            <span className="w-1.5 h-1.5 rounded-full bg-matrix-500" /> live output — htool v{rel.tag.replace('v', '')}
          </div>
        </div>
      </div>
    </section>
  )
}

// ─── Marquee ───────────────────────────────────────────────

function Marquee() {
  const items = ['PORT SCANNING', 'SQL INJECTION', 'XSS DETECTION', 'DIR BRUTEFORCE', 'SUBDOMAIN ENUM', 'SSL ANALYSIS', 'SECURITY HEADERS', 'WAF DETECTION', 'TECH FINGERPRINT', 'SUBDOMAIN TAKEOVER', 'DNS AXFR CHECK', 'CVE MATCHING', 'REVERSE SHELLS', 'WEB SHELLS', 'HTML REPORTS', 'JSON EXPORT']
  return (
    <div className="marquee border-y border-white/5 py-4 bg-cyber-row/40">
      <div className="marquee-track">
        {[...items, ...items].map((t, i) => (
          <span key={i} className="text-xs font-mono tracking-[0.2em] text-cyber-faint whitespace-nowrap flex items-center gap-3">
            <span className="text-matrix-500/70">◆</span> {t}
          </span>
        ))}
      </div>
    </div>
  )
}

// ─── Features (bento) ──────────────────────────────────────

function Features() {
  const big = {
    icon: IconScanner, title: 'Vulnerability Scanner',
    desc: 'One command, a full assessment: port scanning, SQLi & XSS detection, directory brute-forcing, subdomain enumeration, SSL/TLS analysis, security-header audits, WAF detection and offline CVE matching — all multi-threaded with rate limiting and proxy support.',
    chips: ['20+ quick ports / 1-1024 full', '11 SQLi payloads', '12 XSS payloads', 'AXFR + takeover checks'],
  }
  const rest = [
    { icon: IconStress, title: 'Stress Testing', desc: 'HTTP flood, Slowloris, UDP & SYN simulation with per-module tuning.' },
    { icon: IconCredential, title: 'Credential Stuffing', desc: 'Mass login audits with wordlists, proxy rotation & success heuristics.' },
    { icon: IconSpam, title: 'Spam & Flood', desc: 'DB flood, comment & registration spam — rate-limit testing suite.' },
    { icon: IconPayload, title: 'Payload Generator', desc: 'Reverse & bind shells, PHP web shells, download-exec across 7 platforms.' },
    { icon: IconCve, title: 'Offline CVE Database', desc: '50+ critical CVEs bundled — zero internet required to match.' },
    { icon: IconHtmlViewer, title: 'In-App Report Viewer', desc: 'NEW — open scan JSON and read the full rendered report inside the GUI. Export pixel-perfect HTML or raw JSON.' },
    { icon: IconWaf, title: 'WAF Detection', desc: 'Identifies Cloudflare, Sucuri, ModSecurity, AWS WAF, Imperva, F5 & more.' },
  ]

  return (
    <section id="features" className="py-20 md:py-28 relative">
      <div className="max-w-6xl mx-auto px-4 sm:px-6">
        <div className="reveal text-center mb-14">
          <span className="text-xs font-mono text-cyber-cyan bg-cyber-cyan/10 px-3.5 py-1.5 rounded-full border border-cyber-cyan/25">● CAPABILITIES</span>
          <h2 className="font-display text-3xl sm:text-4xl md:text-5xl font-bold text-white mt-5 mb-4">One binary. <span className="text-gradient">Every angle.</span></h2>
          <p className="text-cyber-dim max-w-2xl mx-auto">Each module runs async on Tokio with global rate limiting — fast enough for full scans, gentle enough to stay under the radar.</p>
        </div>

        <div className="grid md:grid-cols-3 gap-5">
          <div className="reveal md:col-span-2 card-glow bg-cyber-card rounded-2xl p-7 border border-white/8">
            <div className="flex items-start justify-between mb-4 flex-wrap gap-4">
              <div className="w-12 h-12"><big.icon className="w-full h-full" /></div>
              <span className="text-[10px] font-mono text-matrix-500 bg-matrix-500/10 border border-matrix-500/30 px-2.5 py-1 rounded-full">CORE MODULE</span>
            </div>
            <h3 className="text-white font-bold text-xl mb-2.5">{big.title}</h3>
            <p className="text-cyber-dim text-sm leading-relaxed mb-5">{big.desc}</p>
            <div className="flex flex-wrap gap-2">
              {big.chips.map(c => (
                <span key={c} className="text-[11px] font-mono text-cyber-cyan bg-cyber-cyan/8 border border-cyber-cyan/20 px-3 py-1 rounded-full">{c}</span>
              ))}
            </div>
          </div>
          <div className="reveal card-glow bg-cyber-card rounded-2xl p-6 border border-white/8">
            <div className="w-12 h-12 mb-4"><IconReport className="w-full h-full" /></div>
            <h3 className="text-white font-bold text-lg mb-2.5">Beautiful Reports</h3>
            <p className="text-cyber-dim text-sm leading-relaxed">Severity-badged, stat-gridded HTML reports with neon chips — generated locally, no telemetry, ever.</p>
          </div>
          {rest.map((f, i) => (
            <div key={i} className="reveal card-glow bg-cyber-card rounded-2xl p-6 border border-white/8">
              <div className="w-10 h-10 mb-4"><f.icon className="w-full h-full" /></div>
              <h3 className="text-white font-bold text-[15px] mb-2">{f.title}</h3>
              <p className="text-cyber-dim text-[13px] leading-relaxed">{f.desc}</p>
            </div>
          ))}
        </div>
      </div>
    </section>
  )
}

// ─── App UI showcase (CSS mock of the new GUI) ─────────────

function AppShowcase() {
  const sidebar = [['◈', 'Dashboard', false], ['◉', 'Scanner', false], ['⏱', 'Scheduler', true], ['⚡', 'Stress Test', false], ['@', 'Cred Stuffing', false], ['✉', 'Spam & Flood', false], ['>_', 'Payload Gen', false], ['▤', 'Report Viewer', false], ['☰', 'CVE Database', false]]
  const stats = [['4', 'OPEN PORTS', 'text-matrix-500'], ['2', 'SQL INJECTION', 'text-red-400'], ['1', 'XSS', 'text-red-400'], ['0', 'TAKEOVERS', 'text-red-400'], ['3', 'TECHNOLOGIES', 'text-cyber-purple']]

  return (
    <section id="app" className="py-20 md:py-28 relative bg-cyber-row/30 border-y border-white/5">
      <div className="max-w-6xl mx-auto px-4 sm:px-6">
        <div className="reveal text-center mb-14">
          <span className="text-xs font-mono text-matrix-500 bg-matrix-500/10 px-3.5 py-1.5 rounded-full border border-matrix-500/25">● THE NEW APP</span>
          <h2 className="font-display text-3xl sm:text-4xl md:text-5xl font-bold text-white mt-5 mb-4">A dashboard that <span className="text-gradient">feels premium.</span></h2>
          <p className="text-cyber-dim max-w-2xl mx-auto">v3.2 ships a fully redesigned GUI — deep-navy theme, neon-emerald accents, stat cards, severity badges and a built-in report viewer that renders your scan results right inside the app. New in v3.2: scheduled scans, PDF reports and a light/dark theme switch.</p>
        </div>

        <div className="reveal max-w-4xl mx-auto">
          <div className="app-mock">
            {/* window bar */}
            <div className="flex items-center gap-2 px-4 py-2.5 border-b border-white/5 bg-cyber-sidebar">
              <span className="w-2.5 h-2.5 rounded-full bg-[#f87171]" />
              <span className="w-2.5 h-2.5 rounded-full bg-[#fbbf24]" />
              <span className="w-2.5 h-2.5 rounded-full bg-[#34d399]" />
              <span className="text-[11px] font-mono text-cyber-faint ml-3">htool — Ultimate Hacker Toolkit</span>
              <span className="ml-auto text-[10px] font-mono text-matrix-500 bg-matrix-500/10 border border-matrix-500/25 px-2 py-0.5 rounded-full">MODULE · SCHEDULER</span>
            </div>
            <div className="grid grid-cols-[120px_1fr] sm:grid-cols-[170px_1fr] min-h-[380px]">
              {/* sidebar */}
              <div className="bg-cyber-sidebar border-r border-white/5 p-2.5 hidden sm:block">
                <div className="text-[9px] font-mono text-cyber-faint tracking-widest px-2 mb-2">MODULES</div>
                {sidebar.map(([icon, label, active]) => (
                  <div key={label} className={`flex items-center gap-2 px-2.5 py-[7px] rounded-lg mb-0.5 text-[11px] font-mono ${active ? 'bg-matrix-500/12 text-cyber-bright border-l-2 border-matrix-500' : 'text-cyber-faint'}`}>
                    <span className={active ? 'text-matrix-500' : ''}>{icon}</span> {label}
                  </div>
                ))}
                <div className="absolute mt-6 text-[9px] font-mono text-cyber-faint/60 px-2">authorised use only</div>
              </div>
              {/* main */}
              <div className="p-4 sm:p-5">
                <div className="flex items-center gap-2 mb-4">
                  <span className="text-[10px] font-mono text-red-300 bg-red-400/10 border border-red-400/30 px-2.5 py-1 rounded-full font-bold">HIGH SEVERITY</span>
                  <span className="text-[10px] font-mono text-cyber-faint">scan · target.com</span>
                </div>
                <div className="grid grid-cols-5 gap-2 mb-4">
                  {stats.map(([n, l, c]) => (
                    <div key={l} className="bg-cyber-card/90 border border-white/5 rounded-lg py-2.5 text-center">
                      <div className={`text-lg font-bold font-mono ${c}`}>{n}</div>
                      <div className="text-[7.5px] tracking-wider text-cyber-faint">{l}</div>
                    </div>
                  ))}
                </div>
                <div className="bg-cyber-card/90 border border-white/5 rounded-xl p-3.5 mb-3">
                  <div className="text-[11px] font-semibold text-cyber-bright mb-2.5">🔓 Open Ports</div>
                  <div className="flex flex-wrap gap-1.5">
                    {[['22 · SSH', 'text-cyber-cyan bg-cyber-cyan/10'], ['80 · HTTP', 'text-cyber-cyan bg-cyber-cyan/10'], ['443 · HTTPS', 'text-cyber-cyan bg-cyber-cyan/10'], ['3306 · MySQL', 'text-cyber-cyan bg-cyber-cyan/10']].map(([t, c]) => (
                      <span key={t} className={`text-[10px] font-mono px-2.5 py-0.5 rounded-full ${c}`}>{t}</span>
                    ))}
                  </div>
                </div>
                <div className="bg-cyber-card/90 border border-red-400/15 border-l-2 border-l-red-400/60 rounded-xl p-3.5 mb-3">
                  <div className="text-[11px] font-semibold text-cyber-bright mb-2">🐍 SQL Injection <span className="text-cyber-faint font-normal">(2)</span></div>
                  {['⚠  target.com/item?id=1', '⚠  target.com/product?cat=2'].map(u => (
                    <div key={u} className="text-[10px] font-mono text-red-300 bg-cyber-deep/90 rounded-md px-2.5 py-1.5 mb-1.5">{u}</div>
                  ))}
                </div>
                <div className="bg-cyber-card/90 border border-white/5 rounded-xl p-3.5">
                  <div className="text-[11px] font-semibold text-cyber-bright mb-2">🛠 Detected Technologies <span className="text-cyber-faint font-normal">(3)</span></div>
                  <div className="flex flex-wrap gap-1.5">
                    {[['Nginx', 'text-cyber-cyan bg-cyber-cyan/10'], ['PHP 8.1', 'text-cyber-cyan bg-cyber-cyan/10'], ['[WAF] Cloudflare WAF', 'text-cyber-orange bg-cyber-orange/10']].map(([t, c]) => (
                      <span key={t} className={`text-[10px] font-mono px-2.5 py-0.5 rounded-full ${c}`}>{t}</span>
                    ))}
                  </div>
                </div>
              </div>
            </div>
          </div>
          <p className="text-center text-cyber-faint text-xs mt-4 font-mono">▲ actual v3.2 GUI — stat grid, severity badges, rendered report preview, scheduler</p>
        </div>
      </div>
    </section>
  )
}

// ─── Download ──────────────────────────────────────────────

function Download() {
  const os = useOs()
  const rel = useLatestRelease()
  const cards = [
    { os: 'windows', label: 'Windows', icon: OsWindows, file: 'htool-x86_64-pc-windows-msvc.zip', ext: '.zip · exe', note: 'htool-gui.exe + htool.exe · Win 10/11 x64', primary: true },
    { os: 'linux', label: 'Linux', icon: OsLinux, file: 'htool-x86_64-unknown-linux-gnu.tar.gz', ext: '.tar.gz', note: 'htool-gui + htool (CLI) · x86_64' },
    { os: 'macos', label: 'macOS', icon: OsMacos, file: 'htool-x86_64-apple-darwin.tar.gz', ext: '.tar.gz', note: 'htool-gui + htool (CLI) · Intel' },
  ]

  return (
    <section id="download" className="py-20 md:py-28 relative overflow-hidden">
      <div className="absolute inset-0 bg-grid opacity-60" />
      <div className="max-w-6xl mx-auto px-4 sm:px-6 text-center relative z-10">
        <div className="reveal">
          <span className="text-xs font-mono text-matrix-500 bg-matrix-500/10 px-3.5 py-1.5 rounded-full border border-matrix-500/25">● DOWNLOAD</span>
          <h2 className="font-display text-3xl sm:text-4xl md:text-5xl font-bold text-white mt-5 mb-4">Get <span className="text-gradient">htool</span></h2>
          <p className="text-cyber-dim max-w-xl mx-auto mb-10">Free & open source. The Windows build ships both the modern GUI and the CLI — unzip and run, no installer needed.</p>
        </div>

        <div className="grid md:grid-cols-3 gap-5 max-w-4xl mx-auto mb-10">
          {cards.map((d, i) => {
            const isActive = os === d.os
            return (
              <div key={i} className={`reveal card-glow bg-cyber-card rounded-2xl p-7 border transition-all ${isActive ? 'border-matrix-500/60 shadow-glow-accent' : 'border-white/8'}`}>
                {isActive && <span className="float-right text-[9px] font-mono text-black bg-matrix-500 px-2 py-0.5 rounded-full font-bold">YOUR OS</span>}
                <div className="w-14 h-14 mx-auto mb-4"><d.icon className="w-full h-full" /></div>
                <h3 className="text-lg font-bold text-white mb-1">{d.label}</h3>
                <p className="text-cyber-faint text-xs font-mono mb-5 break-all">{d.file}</p>
                <p className="text-cyber-dim text-[12.5px] mb-5">{d.note}</p>
                <a href={ASSETS[d.os]} className={`block w-full py-3 rounded-xl font-bold text-sm transition-all btn-flash ${isActive ? 'bg-matrix-500 text-black hover:bg-matrix-400 hover:shadow-glow-accent' : 'bg-white/8 text-cyber-bright hover:bg-white/12'}`}>
                  ⬇ Download {d.ext}
                </a>
              </div>
            )
          })}
        </div>

        <div className="reveal inline-flex items-center gap-3 text-sm text-cyber-dim bg-cyber-card/70 border border-white/8 rounded-full px-5 py-2.5 mb-12">
          <span className="w-2 h-2 rounded-full bg-matrix-500 animate-pulse" />
          latest release <span className="font-mono text-matrix-500 font-semibold">{rel.tag}</span>
          <span className="text-cyber-faint">·</span>
          <a href={RELEASES_PAGE} target="_blank" rel="noopener noreferrer" className="text-cyber-cyan hover:text-matrix-500 transition-colors">view on GitHub →</a>
        </div>

        <div className="reveal terminal-window max-w-xl mx-auto text-left">
          <div className="terminal-header">
            <div className="terminal-dot" style={{ background: '#f87171' }} />
            <div className="terminal-dot" style={{ background: '#fbbf24' }} />
            <div className="terminal-dot" style={{ background: '#34d399' }} />
            <span className="text-cyber-faint text-xs ml-2 font-mono">install</span>
          </div>
          <div className="terminal-content p-4 sm:p-5 text-[12.5px] sm:text-[13px]">
            <div className="text-cyber-faint"># Rust users — install straight from crates.io</div>
            <div><span className="text-matrix-500">$</span> <span className="text-white font-bold">cargo install htool</span></div>
            <div className="mt-3 text-cyber-faint"># or build the GUI from source</div>
            <div><span className="text-matrix-500">$</span> <span className="text-white">git clone https://github.com/Resolutefemi/hacker_toolkit.git</span></div>
            <div><span className="text-matrix-500">$</span> <span className="text-white">cd hacker_toolkit && cargo build --release</span></div>
          </div>
        </div>
      </div>
    </section>
  )
}

// ─── FAQ ───────────────────────────────────────────────────

function FAQ() {
  const faqs = [
    ['Is htool free?', 'Yes — htool is fully open source under the MIT license. The Windows zip includes both the desktop GUI (htool-gui.exe) and the CLI (htool.exe). No accounts, no telemetry, no paywalls.'],
    ['What changed in v3.2?', 'On top of the v3.1 redesign (modern dark-navy GUI, in-app Report Viewer, richer CLI), v3.2 adds: a Scan Scheduler that runs scans automatically (every N minutes or daily) and writes HTML + JSON + PDF reports, one-click PDF report export from the app and CLI, and a light/dark theme toggle that remembers your choice.'],
    ['Which Windows build do I download?', 'Download htool-x86_64-pc-windows-msvc.zip from the latest release. Right-click → Extract all, then run htool-gui.exe for the dashboard or htool.exe from a terminal for the CLI. SmartScreen may warn on first run — click "More info" → "Run anyway".'],
    ['Is this legal to use?', 'Only against systems you own or have explicit written permission to test. htool is built for authorised security assessments, CTF practice and education. Unauthorised use against third-party systems is illegal.'],
    ['Does the scanner need an internet connection?', 'The scanner needs network access to your target, but the CVE database is bundled offline inside the binary — CVE matching, payload generation and reporting all work fully offline.'],
  ]
  return (
    <section id="faq" className="py-20 md:py-28 bg-cyber-row/30 border-y border-white/5">
      <div className="max-w-3xl mx-auto px-4 sm:px-6">
        <div className="reveal text-center mb-12">
          <span className="text-xs font-mono text-cyber-purple bg-cyber-purple/10 px-3.5 py-1.5 rounded-full border border-cyber-purple/25">● FAQ</span>
          <h2 className="font-display text-3xl sm:text-4xl font-bold text-white mt-5">Questions, answered.</h2>
        </div>
        <div className="space-y-3">
          {faqs.map(([q, a], i) => (
            <details key={i} className="reveal group bg-cyber-card border border-white/8 rounded-xl overflow-hidden card-glow">
              <summary className="flex items-center justify-between cursor-pointer list-none px-5 py-4 text-white font-semibold text-[15px]">
                {q}
                <span className="faq-chevron text-matrix-500 text-lg group-open:rotate-180 transition-transform">⌄</span>
              </summary>
              <p className="px-5 pb-5 text-cyber-dim text-sm leading-relaxed">{a}</p>
            </details>
          ))}
        </div>
      </div>
    </section>
  )
}

// ─── Author ────────────────────────────────────────────────

function Author() {
  return (
    <section id="author" className="py-20 md:py-28">
      <div className="max-w-4xl mx-auto px-4 text-center">
        <div className="reveal bg-cyber-card rounded-2xl p-8 sm:p-12 border border-white/8 card-glow">
          <div className="w-24 h-24 mx-auto mb-6 rounded-2xl bg-gradient-to-br from-matrix-500 to-cyber-cyan flex items-center justify-center p-5 shadow-glow-accent">
            <HIcon className="w-full h-full" />
          </div>
          <h2 className="font-display text-2xl sm:text-3xl font-bold text-white mb-2">Built by <span className="text-gradient">Resolute Femi</span></h2>
          <p className="text-matrix-500 font-mono text-sm mb-5">@Resolutefemi</p>
          <p className="text-cyber-dim max-w-xl mx-auto leading-relaxed text-[15px]">
            Security researcher and Rust developer. htool is a comprehensive, all-in-one toolkit for
            checking vulnerabilities and testing security posture — engineered for performance, polished for people.
          </p>
          <div className="flex justify-center gap-4 mt-8">
            <a href="https://github.com/Resolutefemi" target="_blank" rel="noopener noreferrer" className="border border-white/15 hover:border-matrix-500/60 hover:text-matrix-500 text-cyber-dim transition-all px-5 py-2.5 rounded-xl text-sm font-semibold">GitHub →</a>
            <a href={RELEASES_PAGE} target="_blank" rel="noopener noreferrer" className="border border-white/15 hover:border-matrix-500/60 hover:text-matrix-500 text-cyber-dim transition-all px-5 py-2.5 rounded-xl text-sm font-semibold">Releases →</a>
          </div>
        </div>
      </div>
    </section>
  )
}

// ─── Footer ────────────────────────────────────────────────

function Footer() {
  return (
    <footer className="border-t border-white/5 py-12 bg-cyber-sidebar/60">
      <div className="max-w-6xl mx-auto px-4">
        <div className="flex flex-col md:flex-row justify-between items-center gap-6">
          <div className="flex items-center gap-3">
            <div className="w-8 h-8 rounded-lg bg-matrix-500/15 border border-matrix-500/40 flex items-center justify-center">
              <HIcon className="w-5 h-5" />
            </div>
            <span className="text-white font-bold text-lg font-mono">htool</span>
            <span className="text-cyber-faint text-sm">by Resolute Femi</span>
          </div>
          <div className="flex gap-6 text-sm text-cyber-dim">
            <a href="#features" className="hover:text-matrix-500 transition-colors">Features</a>
            <a href="#download" className="hover:text-matrix-500 transition-colors">Download</a>
            <a href={RELEASES_PAGE} target="_blank" rel="noopener noreferrer" className="hover:text-matrix-500 transition-colors">Releases</a>
            <a href="https://github.com/Resolutefemi/hacker_toolkit" target="_blank" rel="noopener noreferrer" className="hover:text-matrix-500 transition-colors">Source</a>
          </div>
        </div>
        <div className="mt-8 pt-8 border-t border-white/5 text-center text-cyber-faint text-xs leading-relaxed">
          <p>⚠️ For authorised security testing and educational purposes only. Unauthorised use against systems you do not own is illegal.</p>
          <p className="mt-2">© {new Date().getFullYear()} Resolute Femi · htool v3.2.0 · built with Rust, Tokio & egui</p>
        </div>
      </div>
    </footer>
  )
}

// ─── Page ──────────────────────────────────────────────────

export default function Home() {
  useReveal()
  return (
    <>
      <Head>
        <title>htool — Modern Cybersecurity Toolkit by Resolute Femi | Free Download</title>
        <meta name="description" content="Download htool v3.1 by Resolute Femi — the redesigned all-in-one cybersecurity toolkit. Vulnerability scanning, stress testing, payload generation and a beautiful new GUI with in-app report viewing. Built with Rust, free & open source." />
      </Head>

      <main className="bg-cyber-dark min-h-screen overflow-x-hidden">
        <Header />
        <Hero />
        <Marquee />
        <Features />
        <AppShowcase />
        <Download />
        <FAQ />
        <Author />
        <Footer />
      </main>
    </>
  )
}
