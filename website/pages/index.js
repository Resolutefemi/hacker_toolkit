import Head from 'next/head'
import { useState, useEffect } from 'react'
import {
  IconScanner, IconStress, IconCredential, IconSpam,
  IconPayload, IconReport, IconCve, IconHtmlViewer, IconWaf,
  OsWindows, OsMacos, OsLinux, HIcon
} from '../components/icons'

// ─── OS Detection Hook ───
function useOs() {
  const [os, setOs] = useState('linux')
  useEffect(() => {
    const ua = navigator.userAgent.toLowerCase()
    if (ua.includes('win')) setOs('windows')
    else if (ua.includes('mac')) setOs('macos')
    else setOs('linux')
  }, [])
  return os
}

// ─── Header ───
function Header() {
  const [menuOpen, setMenuOpen] = useState(false)

  return (
    <header className="fixed top-0 left-0 right-0 z-50 bg-cyber-dark/95 backdrop-blur-lg border-b border-matrix-500/20">
      <div className="max-w-6xl mx-auto px-4 h-14 sm:h-16 flex items-center justify-between">
        <div className="flex items-center gap-2 sm:gap-3">
          <div className="flex items-center gap-2">
            <HIcon className="w-7 h-7 sm:w-8 sm:h-8" />
            <span className="text-matrix-500 text-xl sm:text-2xl font-bold font-mono">htool</span>
          </div>
          <span className="text-cyber-dim text-xs hidden sm:block font-mono">v3.0</span>
        </div>

        {/* Desktop nav */}
        <nav className="hidden md:flex items-center gap-6">
          <a href="#features" className="text-cyber-dim hover:text-matrix-500 transition-colors text-sm">Features</a>
          <a href="#download" className="text-cyber-dim hover:text-matrix-500 transition-colors text-sm">Download</a>
          <a href="#author" className="text-cyber-dim hover:text-matrix-500 transition-colors text-sm">Author</a>
          <a href="#download" className="bg-matrix-500 text-black px-4 py-2 rounded font-semibold text-sm hover:bg-matrix-400 transition-all btn-flash">
            Get htool
          </a>
        </nav>

        {/* Mobile hamburger */}
        <button
          onClick={() => setMenuOpen(!menuOpen)}
          className="md:hidden flex flex-col gap-1.5 p-2 rounded hover:bg-matrix-500/10 transition-colors"
          aria-label="Toggle menu"
        >
          <span className={`block w-6 h-0.5 bg-matrix-500 transition-all ${menuOpen ? 'rotate-45 translate-y-2' : ''}`} />
          <span className={`block w-6 h-0.5 bg-matrix-500 transition-all ${menuOpen ? 'opacity-0' : ''}`} />
          <span className={`block w-6 h-0.5 bg-matrix-500 transition-all ${menuOpen ? '-rotate-45 -translate-y-2' : ''}`} />
        </button>
      </div>

      {/* Mobile menu overlay */}
      <div className={`md:hidden fixed inset-0 bg-cyber-dark/98 backdrop-blur-xl z-40 transition-all duration-300 ${menuOpen ? 'opacity-100 pointer-events-auto' : 'opacity-0 pointer-events-none'}`}>
        <div className="flex flex-col items-center justify-center h-full gap-8">
          <a
            href="#features"
            onClick={() => setMenuOpen(false)}
            className="text-2xl text-cyber-dim hover:text-matrix-500 transition-colors font-mono"
          >
            [ Features ]
          </a>
          <a
            href="#download"
            onClick={() => setMenuOpen(false)}
            className="text-2xl text-cyber-dim hover:text-matrix-500 transition-colors font-mono"
          >
            [ Download ]
          </a>
          <a
            href="#author"
            onClick={() => setMenuOpen(false)}
            className="text-2xl text-cyber-dim hover:text-matrix-500 transition-colors font-mono"
          >
            [ Author ]
          </a>
          <a
            href="#download"
            onClick={() => setMenuOpen(false)}
            className="mt-4 bg-matrix-500 text-black px-10 py-4 rounded-lg font-bold text-xl hover:bg-matrix-400 transition-all btn-flash"
          >
            Get htool
          </a>
        </div>
      </div>
    </header>
  )
}

// ─── Hero ───
function Hero() {
  return (      <section className="min-h-[90vh] sm:min-h-screen flex items-center justify-center relative pt-14 sm:pt-16">
      <div className="max-w-6xl mx-auto px-4 sm:px-6 text-center relative z-10">
        <h1 className="text-4xl sm:text-5xl md:text-7xl font-bold mb-4 sm:mb-6">
          <span className="text-matrix-500">htool</span>
          <br />
          <span className="text-cyber-bright text-2xl sm:text-3xl md:text-4xl font-light block mt-2 sm:mt-0">
            Cybersecurity Toolkit
          </span>
        </h1>
        <p className="text-cyber-dim text-base sm:text-lg md:text-xl max-w-3xl mx-auto mb-6 sm:mb-8 font-light px-2 sm:px-0">
          The ultimate all-in-one security testing framework by{' '}
          <span className="text-matrix-500 font-semibold">Resolute Femi</span>.
          Built with Rust for speed. Features advanced <strong className="text-cyber-bright">tools for checking vulnerability</strong>,
          stress testing, credential analysis, and payload generation.
        </p>
        <div className="flex flex-col sm:flex-row gap-3 sm:gap-4 justify-center mb-8 sm:mb-12 px-4 sm:px-0">
          <a href="#download" className="bg-matrix-500 text-black px-6 sm:px-8 py-3 sm:py-4 rounded-lg font-bold text-base sm:text-lg hover:bg-matrix-400 transition-all btn-flash shadow-lg shadow-matrix-500/25 w-full sm:w-auto text-center">
            ⬇ Download Now
          </a>
          <a href="#features" className="border border-cyber-cyan/50 text-cyber-cyan px-6 sm:px-8 py-3 sm:py-4 rounded-lg font-semibold text-base sm:text-lg hover:bg-cyber-cyan/10 transition-all w-full sm:w-auto text-center">
            View Features →
          </a>
        </div>

        {/* Terminal Demo */}
        <div className="terminal-window max-w-2xl mx-2 sm:mx-auto text-left">
          <div className="terminal-header px-3 sm:px-4">
            <div className="terminal-dot w-2.5 h-2.5 sm:w-3 sm:h-3" style={{ background: '#ff0055' }} />
            <div className="terminal-dot w-2.5 h-2.5 sm:w-3 sm:h-3" style={{ background: '#ffa500' }} />
            <div className="terminal-dot w-2.5 h-2.5 sm:w-3 sm:h-3" style={{ background: '#00ff41' }} />
            <span className="text-cyber-dim text-2xs sm:text-xs ml-1 sm:ml-2 font-mono">htool@scan:~</span>
          </div>
          <div className="terminal-content p-3 sm:p-4 overflow-x-auto">
            <span className="text-matrix-500">$</span>{' '}
            <span className="text-cyber-bright">htool scan</span>{' '}
            <span className="text-cyber-cyan">example.com</span>
            <br />
            <span className="text-cyber-dim">🔍 Starting scan...</span>
            <br />
            <span className="text-matrix-500">✅ Open ports:</span> 22 80 443 3306
            <br />
            <span className="text-cyber-cyan">🐍 SQLi:</span> 2 URLs
            <br />
            <span className="text-cyber-orange">🕸️ XSS:</span> 1 URL
            <br />
            <span className="text-matrix-500">🛠️ Services:</span> Nginx, PHP, WP
            <br />
            <span className="text-cyber-dim mt-1 sm:mt-2 block">Scan: 12.4s</span>
          </div>
        </div>
      </div>
    </section>
  )
}

// ─── Features ───
function Features() {
  const features = [
    { icon: IconScanner, title: 'Vulnerability Scanner', desc: 'Port scanning, SQLi, XSS, directory brute-forcing, subdomain enumeration, SSL/TLS analysis, security headers audit, and offline CVE matching with 100+ technology fingerprint signatures.' },
    { icon: IconStress, title: 'Stress Testing', desc: 'HTTP flood, Slowloris, UDP flood, SYN flood simulation, advanced HTTP with random methods. Full authorised testing suite.' },
    { icon: IconCredential, title: 'Credential Stuffing', desc: 'Mass login testing with wordlists, proxy rotation, rate limiting, and detailed result logging with success detection.' },
    { icon: IconSpam, title: 'Spam & Flood', desc: 'Database flooding, email bomber, SMS bomber, comment spam, and registration spam with real HTTP request support.' },
    { icon: IconPayload, title: 'Payload Generator', desc: 'Reverse shells for Linux/Windows/Python/PHP/Node.js/Ruby/Perl, bind shells, PHP web shells, download & execute payloads.' },
    { icon: IconReport, title: 'Report Generation', desc: 'Beautiful HTML and JSON reports with full vulnerability details, technology stack analysis, and export to file.' },
    { icon: IconCve, title: 'CVE Database', desc: 'Built-in offline CVE database with 50+ entries. Search by product, version, keyword, or CVSS score.' },
    { icon: IconHtmlViewer, title: 'HTML Viewer', desc: 'Built-in HTML report viewer with preview, save, and open in browser functionality.' },
    { icon: IconWaf, title: 'WAF Detection', desc: 'Active Web Application Firewall detection. Identifies Cloudflare, Sucuri, ModSecurity, AWS WAF, Imperva, and more.' },
  ]

  return (
    <section id="features" className="py-16 md:py-24 relative">
      <div className="max-w-6xl mx-auto px-4 sm:px-6">
        <div className="text-center mb-16">
          <span className="text-xs font-mono text-cyber-cyan bg-cyber-cyan/10 px-3 py-1 rounded-full border border-cyber-cyan/30">
            ● FEATURES
          </span>
          <h2 className="text-2xl sm:text-3xl md:text-5xl font-bold text-cyber-bright mt-4 mb-4">
            Everything You Need
          </h2>
          <p className="text-cyber-dim text-base sm:text-lg max-w-2xl mx-auto px-2 sm:px-0">
            htool by <strong className="text-matrix-500">Resolute Femi</strong> provides
            professional-grade <strong className="text-cyber-cyan">tools for checking vulnerability</strong>
            {' '}and securing your infrastructure.
          </p>
        </div>
        <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
          {features.map((f, i) => (
            <div key={i} className="bg-cyber-card rounded-xl p-4 sm:p-6 border border-matrix-500/20 hover:border-matrix-500/40 transition-all group">
              <div className="relative z-10">
                <div className="mb-4 w-10 h-10"><f.icon className="w-full h-full" /></div>
                <h3 className="text-cyber-bright font-bold text-lg mb-2 group-hover:text-matrix-500 transition-colors">
                  {f.title}
                </h3>
                <p className="text-cyber-dim text-sm leading-relaxed">{f.desc}</p>
              </div>
            </div>
          ))}
        </div>
      </div>
    </section>
  )
}

// ─── Tech Stack ───
function TechStack() {
  const techs = ['Rust', 'Tokio', 'egui', 'reqwest', 'Hickory DNS', 'Clap', 'mimalloc', 'Async/Await']
  return (
    <section className="py-12 md:py-16 relative">
      <div className="max-w-6xl mx-auto px-4 sm:px-6 text-center">
        <span className="text-xs font-mono text-cyber-orange bg-cyber-orange/10 px-3 py-1 rounded-full border border-cyber-orange/30">
          ● BUILT WITH RUST
        </span>
        <h2 className="text-xl sm:text-2xl md:text-4xl font-bold text-cyber-bright mt-4 mb-6 sm:mb-8">
          Powered By Modern Technology
        </h2>
        <div className="flex flex-wrap justify-center gap-4">
          {techs.map((t, i) => (
            <span key={i} className="text-sm font-mono text-matrix-500/70 bg-matrix-500/5 px-4 py-2 rounded-lg border border-matrix-500/20">
              {t}
            </span>
          ))}
        </div>
        <p className="text-cyber-dim mt-6 sm:mt-8 max-w-2xl mx-auto px-2 sm:px-0 text-sm sm:text-base">
          Written entirely in <strong className="text-cyber-bright">Rust</strong> for maximum performance, memory safety, and zero-cost abstractions.
          Async I/O with Tokio for concurrent scanning, egui for the desktop GUI.
        </p>
      </div>
    </section>
  )
}

// ─── Download ───
function Download() {
  const os = useOs()
  const downloads = [
    { os: 'windows', label: 'Windows', icon: OsWindows, desc: 'htool-gui.exe + htool.exe', coming: false, url: 'https://github.com/Resolutefemi/hacker_toolkit/releases/latest/download/htool-x86_64-pc-windows-msvc.zip' },
    { os: 'linux', label: 'Linux', icon: OsLinux, desc: 'htool-gui + htool (CLI)', coming: false, url: 'https://github.com/Resolutefemi/hacker_toolkit/releases/latest/download/htool-x86_64-unknown-linux-gnu.tar.gz' },
    { os: 'macos', label: 'macOS', icon: OsMacos, desc: 'htool-gui + htool (CLI)', coming: false, url: 'https://github.com/Resolutefemi/hacker_toolkit/releases/latest/download/htool-x86_64-apple-darwin.tar.gz' },
  ]

  return (
    <section id="download" className="py-16 md:py-24 relative">
      <div className="max-w-6xl mx-auto px-4 text-center relative z-10">
        <span className="text-xs font-mono text-matrix-500 bg-matrix-500/10 px-3 py-1 rounded-full border border-matrix-500/30">
          ● DOWNLOAD
        </span>
        <h2 className="text-2xl sm:text-3xl md:text-5xl font-bold text-cyber-bright mt-4 mb-4">
          Get htool
        </h2>
        <p className="text-cyber-dim text-base sm:text-lg max-w-2xl mx-auto mb-8 sm:mb-12 px-2 sm:px-0">
          Download the latest release. Available for all major platforms.
          Built by <span className="text-matrix-500 font-semibold">Resolute Femi</span>.
        </p>
        <div className="grid md:grid-cols-3 gap-6 max-w-4xl mx-auto">
          {downloads.map((d, i) => {
            const isActive = os === d.os && !d.coming
            return (
              <div key={i} className={`bg-cyber-card rounded-xl p-6 sm:p-8 border transition-all ${isActive ? 'border-matrix-500 shadow-lg shadow-matrix-500/20' : 'border-cyber-dim/20 hover:border-matrix-500/40'}`}>
                <div className="mb-4 w-14 h-14 mx-auto"><d.icon className="w-full h-full" /></div>
                <h3 className="text-xl font-bold text-cyber-bright mb-2">{d.label}</h3>
                <p className="text-cyber-dim text-sm mb-6">{d.desc}</p>
                {d.coming ? (
                  <span className="inline-block bg-cyber-dim/20 text-cyber-dim px-6 py-3 rounded-lg font-semibold cursor-not-allowed">
                    Coming Soon
                  </span>
                ) : (
                  <a href={d.url}
                     target="_blank"
                     rel="noopener noreferrer"
                     className="inline-block bg-matrix-500 text-black px-5 sm:px-6 py-3 rounded-lg font-bold text-sm sm:text-base hover:bg-matrix-400 transition-all btn-flash shadow-lg shadow-matrix-500/25 w-full sm:w-auto">
                    ⬇ {d.label}
                  </a>
                )}
              </div>
            )
          })}
        </div>

        {/* Latest Release Info */}
        <div className="mt-8 sm:mt-12 max-w-lg mx-auto bg-cyber-card rounded-xl p-5 sm:p-6 border border-matrix-500/20">
          <div className="flex items-center gap-2 mb-4">
            <span className="w-2 h-2 rounded-full bg-matrix-500" />
            <h4 className="text-cyber-bright font-bold font-mono text-sm">// LATEST RELEASE</h4>
          </div>
          <div className="flex justify-between text-sm text-cyber-dim mb-2 pb-2 border-b border-cyber-dim/10">
            <span>Version</span>
            <span className="text-matrix-500 font-mono font-semibold">v3.0.0</span>
          </div>
          <div className="flex justify-between text-sm text-cyber-dim mb-2 pb-2 border-b border-cyber-dim/10">
            <span>Build</span>
            <span className="text-cyber-dim font-mono">Release (optimized)</span>
          </div>
          <div className="flex justify-between text-sm text-cyber-dim mb-2 pb-2 border-b border-cyber-dim/10">
            <span>Windows</span>
            <span className="text-matrix-500 font-mono">htool-x86_64.msi</span>
          </div>
          <div className="flex justify-between text-sm text-cyber-dim mb-2 pb-2 border-b border-cyber-dim/10">
            <span>Linux</span>
            <span className="text-cyber-cyan font-mono">htool-x86_64.AppImage</span>
          </div>
          <div className="flex justify-between text-sm text-cyber-dim">
            <span>macOS</span>
            <span className="text-cyber-cyan font-mono">htool-x86_64.dmg</span>
          </div>
        </div>

        {/* Install via Cargo */}
        <div className="mt-8 sm:mt-12 terminal-window max-w-lg mx-2 sm:mx-auto text-left">
          <div className="terminal-header">
            <div className="terminal-dot" style={{ background: '#ff0055' }} />
            <div className="terminal-dot" style={{ background: '#ffa500' }} />
            <div className="terminal-dot" style={{ background: '#00ff41' }} />
            <span className="text-cyber-dim text-xs ml-2 font-mono">install</span>
          </div>
          <div className="terminal-content">
            <span className="text-cyber-dim"># Install via Cargo (Rust package manager)</span>
            <br />
            <span className="text-matrix-500">$</span>{' '}
            <span className="text-cyber-bright">cargo install htool</span>
            <br /><br />
            <span className="text-cyber-dim"># Or build from source</span>
            <br />
            <span className="text-matrix-500">$</span>{' '}
            <span className="text-cyber-bright">git clone https://github.com/Resolutefemi/htool.git</span>
            <br />
            <span className="text-matrix-500">$</span>{' '}
            <span className="text-cyber-bright">cd htool && cargo build --release</span>
          </div>
        </div>
      </div>
    </section>
  )
}

// ─── Author ───
function Author() {
  return (
    <section id="author" className="py-16 md:py-24 relative">
      <div className="max-w-4xl mx-auto px-4 text-center">
        <span className="text-xs font-mono text-cyber-pink bg-cyber-pink/10 px-3 py-1 rounded-full border border-cyber-pink/30">
          ● CREATOR
        </span>
        <h2 className="text-2xl sm:text-3xl md:text-4xl font-bold text-cyber-bright mt-4 mb-6 sm:mb-8">
          Built by <span className="text-matrix-500">Resolute Femi</span>
        </h2>
        <div className="bg-cyber-card rounded-xl p-6 sm:p-8 md:p-12 border border-cyber-dim/20 mx-2 sm:mx-0">
          <div className="w-24 h-24 mx-auto mb-6 rounded-full bg-gradient-to-br from-matrix-500 to-cyber-cyan flex items-center justify-center p-4">
            <HIcon className="w-full h-full" />
          </div>
          <h3 className="text-2xl font-bold text-cyber-bright mb-2">Resolute Femi</h3>
          <p className="text-matrix-500 font-mono mb-4">@Resolutefemi</p>
          <p className="text-cyber-dim text-base sm:text-lg max-w-2xl mx-auto leading-relaxed px-2 sm:px-0">
            htool is crafted by <strong className="text-cyber-bright">Resolute Femi</strong>, a security researcher and Rust developer.
            This project represents a comprehensive set of <strong className="text-cyber-cyan">tools for checking vulnerability</strong>{' '}
            and testing security posture, built with performance and reliability at its core.
          </p>
          <div className="flex justify-center gap-4 mt-8">
            <a href="https://github.com/Resolutefemi" target="_blank" rel="noopener noreferrer" className="text-cyber-dim hover:text-matrix-500 transition-colors">
              GitHub →
            </a>
          </div>
        </div>
      </div>
    </section>
  )
}

// ─── Footer ───
function Footer() {
  return (
    <footer className="border-t border-matrix-500/20 py-12">
      <div className="max-w-6xl mx-auto px-4">
        <div className="flex flex-col md:flex-row justify-between items-center gap-4">
          <div className="flex items-center gap-3">
            <span className="text-matrix-500 font-bold text-xl font-mono">htool</span>
            <span className="text-cyber-dim text-sm">by Resolute Femi</span>
          </div>
          <div className="text-cyber-dim text-sm text-center md:text-right">
            <p>Professional cybersecurity testing framework</p>
            <p className="mt-1">
              Keywords: <span className="text-matrix-500">htool</span>,{' '}
              <span className="text-matrix-500">Resolute Femi</span>,{' '}
              <span className="text-matrix-500">Resolutefemi</span>,{' '}
              <span className="text-cyber-cyan">Tools for Checking Vulnerability</span>
            </p>
          </div>
        </div>
        <div className="mt-8 pt-8 border-t border-cyber-dim/10 text-center text-cyber-dim text-xs">
          <p>⚠️ For authorised security testing and educational purposes only.</p>
          <p className="mt-1">Unauthorised use against systems you do not own is illegal.</p>
          <p className="mt-4">&copy; {new Date().getFullYear()} Resolute Femi. All rights reserved.</p>
        </div>
      </div>
    </footer>
  )
}

// ─── Main Page ───
export default function Home() {
  return (
    <>
      <Head>
        <title>htool — Cybersecurity Toolkit by Resolute Femi | Download Free</title>
        <meta name="description" content="Download htool by Ariyo Oluwafemi Stephen (Resolute Femi), the ultimate all-in-one cybersecurity testing toolkit. Advanced tools for checking vulnerability, network scanning, stress testing, and more. Built with Rust." />
      </Head>

      <main className="bg-cyber-dark min-h-screen">
        <Header />
        <Hero />
        <Features />
        <TechStack />
        <Download />
        <Author />
        <Footer />
      </main>
    </>
  )
}
