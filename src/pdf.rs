//! Dependency-free PDF report writer.
//! Renders a branded A4 vulnerability report using core-14 Helvetica fonts
//! (no font embedding, no external crates). Supports colored banners, stat
//! grids, automatic pagination and word wrapping.

use crate::scanner::ScanResult;
use chrono::Local;

// ─── Page geometry (A4, units = points, origin top-left internally) ───
const PAGE_W: f32 = 595.0;
const PAGE_H: f32 = 842.0;
const M: f32 = 46.0; // page margin
const CW: f32 = PAGE_W - 2.0 * M; // content width
const FOOTER_H: f32 = 30.0;

// ─── Colors (r, g, b in 0.0..1.0) ───
type Rgb = (f32, f32, f32);
const INK: Rgb = (0.10, 0.13, 0.22); // near-black navy
const INK_SOFT: Rgb = (0.31, 0.35, 0.45);
const INK_FAINT: Rgb = (0.55, 0.59, 0.67);
const CARD_BG: Rgb = (0.965, 0.973, 0.984);
const CARD_LINE: Rgb = (0.85, 0.88, 0.93);
const NAVY: Rgb = (0.04, 0.055, 0.10); // header banner
const EMERALD: Rgb = (0.0, 0.68, 0.47);
const CYAN: Rgb = (0.09, 0.51, 0.76);
const RED: Rgb = (0.86, 0.21, 0.21);
const ORANGE: Rgb = (0.91, 0.42, 0.13);
const AMBER: Rgb = (0.72, 0.48, 0.0);
const PURPLE: Rgb = (0.45, 0.26, 0.78);
const WHITE: Rgb = (1.0, 1.0, 1.0);

fn sev_rgb(sev: &str) -> Rgb {
    match sev {
        "CRITICAL" => RED,
        "HIGH" => ORANGE,
        "MEDIUM" => AMBER,
        "LOW" => CYAN,
        _ => EMERALD,
    }
}

// ─── Helvetica AFM widths (per mille) for accurate wrapping ───
const fn default_widths() -> [u16; 96] {
    [
        278, 278, 355, 556, 556, 889, 667, 191, 333, 333, 389, 584, 278, 333, 278, 278, // space..?
        556, 556, 556, 556, 556, 556, 556, 556, 556, 556, 278, 278, 584, 584, 584, 556, // 0-9..@
        1015, 667, 667, 722, 722, 667, 611, 778, 722, 278, 500, 667, 556, 833, 722, 778, // A..O
        667, 778, 722, 667, 611, 722, 667, 944, 667, 667, 611, 278, 278, 278, 469, 556, // P.._
        333, 556, 556, 500, 556, 556, 278, 556, 556, 222, 222, 500, 222, 833, 556, 556, // `..o
        556, 556, 333, 500, 278, 556, 500, 722, 500, 500, 500, 334, 260, 334, 584, 556, // p..~
    ]
}
static WIDTHS: [u16; 96] = default_widths();

fn char_w(c: char) -> u16 {
    let idx = c as usize;
    if (0x20..0x80).contains(&idx) { WIDTHS[idx - 0x20] } else { 556 }
}

fn text_width(s: &str, size: f32, bold: bool) -> f32 {
    let base: u32 = s.chars().map(|c| char_w(c) as u32).sum();
    let w = base as f32 * size / 1000.0;
    if bold { w * 1.07 } else { w }
}

fn wrap_text(s: &str, max_w: f32, size: f32, bold: bool) -> Vec<String> {
    let mut lines = Vec::new();
    for para in s.lines() {
        let mut cur = String::new();
        for word in para.split_whitespace() {
            let candidate = if cur.is_empty() { word.to_string() } else { format!("{} {}", cur, word) };
            if text_width(&candidate, size, bold) <= max_w {
                cur = candidate;
            } else {
                if !cur.is_empty() { lines.push(std::mem::take(&mut cur)); }
                // break very long tokens (URLs) char by char
                if text_width(word, size, bold) > max_w {
                    let mut piece = String::new();
                    for ch in word.chars() {
                        if text_width(&format!("{}{}", piece, ch), size, bold) > max_w && !piece.is_empty() {
                            lines.push(std::mem::take(&mut piece));
                        }
                        piece.push(ch);
                    }
                    cur = piece;
                } else {
                    cur = word.to_string();
                }
            }
        }
        lines.push(cur);
    }
    if lines.is_empty() { lines.push(String::new()); }
    lines
}

// ─── WinAnsi-safe string encoding ───
fn pdf_escape(s: &str) -> String {
    let mut out = String::new();
    for c in s.chars() {
        match c {
            '(' | ')' | '\\' => { out.push('\\'); out.push(c); }
            '\u{2018}' | '\u{2019}' => out.push('\''),
            '\u{201C}' | '\u{201D}' => out.push('"'),
            '\u{2013}' | '\u{2014}' => out.push('-'),
            '\u{2026}' => out.push_str("..."),
            '\u{2713}' | '\u{2714}' => out.push('+'),
            '\u{2717}' | '\u{2718}' => out.push('x'),
            '\u{26A0}' => out.push_str("[!]"),
            '\u{2192}' | '\u{21D2}' => out.push_str("->"),
            '\u{00B7}' | '\u{2022}' => out.push('\u{B7}'),
            '\u{25CF}' | '\u{25CB}' => out.push('\u{B7}'),
            '°' => out.push('°'),
            'é' | 'è' | 'ê' | 'ë' | 'á' | 'à' | 'â' | 'ä' | 'ú' | 'ù' | 'û' | 'ü'
            | 'í' | 'ì' | 'î' | 'ï' | 'ó' | 'ò' | 'ô' | 'ö' | 'ñ' | 'ç' => out.push(c),
            c if (c as u32) < 32 => {}
            c if (c as u32) < 0x7F || ('\u{A0}'..='\u{FF}').contains(&c) => out.push(c),
            _ => {} // strip emoji / unsupported glyphs
        }
    }
    out
}

// ─── Content builder ───
struct Pen {
    pages: Vec<Vec<String>>,
    cur: Vec<String>,
    y: f32, // cursor measured downward from top edge
}

impl Pen {
    fn new() -> Self {
        Self { pages: Vec::new(), cur: Vec::new(), y: 0.0 }
    }
    fn finish_page(&mut self) {
        let done = std::mem::take(&mut self.cur);
        if !done.is_empty() { self.pages.push(done); }
    }
    fn new_page(&mut self) {
        self.finish_page();
        self.y = M;
    }
    fn ensure(&mut self, needed: f32) {
        if self.y + needed > PAGE_H - M - FOOTER_H { self.new_page(); }
    }
    fn set_fill(&mut self, c: Rgb) {
        self.cur.push(format!("{} {} {} rg", f(c.0), f(c.1), f(c.2)));
    }
    fn rect(&mut self, x: f32, y_top: f32, w: f32, h: f32, c: Rgb) {
        self.set_fill(c);
        self.cur.push(format!("{} {} {} {} re f", f(x), f(PAGE_H - y_top - h), f(w), f(h)));
    }
    fn stroke_rect(&mut self, x: f32, y_top: f32, w: f32, h: f32, c: Rgb) {
        self.cur.push(format!("{} {} {} RG", f(c.0), f(c.1), f(c.2)));
        self.cur.push(format!("0.8 w {} {} {} {} re S", f(x), f(PAGE_H - y_top - h), f(w), f(h)));
    }
    fn hline(&mut self, x: f32, y_top: f32, w: f32, c: Rgb) {
        self.cur.push(format!("{} {} {} RG 0.8 w", f(c.0), f(c.1), f(c.2)));
        self.cur.push(format!("{} {} m {} {} l S", f(x), f(PAGE_H - y_top), f(x + w), f(PAGE_H - y_top)));
    }
    /// Draw one line of text at (x, y_top as baseline top reference).
    fn text(&mut self, x: f32, y_top: f32, font: &str, size: f32, c: Rgb, s: &str) {
        self.set_fill(c);
        self.cur.push(format!(
            "BT /{} {} Tf {} {} Td ({}) Tj ET",
            font, f(size), f(x), f(PAGE_H - y_top), pdf_escape(s)
        ));
    }
    /// Wrapped paragraph starting at cursor; advances cursor. Returns height used.
    #[allow(clippy::too_many_arguments)]
    fn para(&mut self, x: f32, max_w: f32, font: &str, size: f32, c: Rgb, s: &str, leading: f32) -> f32 {
        let bold = font == "F2";
        let lines = wrap_text(s, max_w, size, bold);
        for line in &lines {
            self.ensure(leading);
            self.text(x, self.y + size * 0.85, font, size, c, line);
            self.y += leading;
        }
        lines.len() as f32 * leading
    }
}

fn f(v: f32) -> String {
    let v = (v * 100.0).round() / 100.0;
    if v == v.trunc() { format!("{:.0}", v) } else { format!("{:.2}", v) }
}

// ─── Report sections ───

fn section_header(p: &mut Pen, title: &str, count: usize, color: Rgb) {
    p.ensure(30.0);
    p.rect(M, p.y, 3.0, 13.0, color);
    let label = if count > 0 { format!("{}  ({})", title, count) } else { title.to_string() };
    p.text(M + 10.0, p.y, "F2", 11.5, INK, &label);
    p.y += 22.0;
}

fn bullet_list(p: &mut Pen, items: &[String], color: Rgb, empty_note: &str) {
    if items.is_empty() {
        p.text(M + 12.0, p.y + 8.0, "F3", 9.0, EMERALD, &format!("+ {}", empty_note));
        p.y += 20.0;
        return;
    }
    for item in items {
        let lines = wrap_text(item, CW - 26.0, 9.5, false);
        p.ensure(lines.len() as f32 * 13.0 + 4.0);
        p.rect(M + 4.0, p.y + 4.0, 2.6, 2.6, color);
        for (i, line) in lines.iter().enumerate() {
            let x = M + 14.0;
            p.text(x, p.y + i as f32 * 13.0 + 4.0, "F1", 9.5, INK_SOFT, line);
        }
        p.y += lines.len() as f32 * 13.0 + 4.0;
    }
    p.y += 6.0;
}

fn mono_block(p: &mut Pen, content: &str, max_lines_cap: usize) {
    if content.trim().is_empty() {
        p.text(M + 12.0, p.y + 8.0, "F3", 9.0, INK_FAINT, "no data captured");
        p.y += 20.0;
        return;
    }
    let lines: Vec<String> = content.lines().take(max_lines_cap).map(|l| l.to_string()).collect();
    let wrapped: Vec<String> = lines.iter().flat_map(|l| wrap_text(l, CW - 28.0, 8.3, false).into_iter()).collect();
    let h = wrapped.len() as f32 * 11.5 + 14.0;
    p.ensure(h);
    p.rect(M + 4.0, p.y, CW - 4.0, h, CARD_BG);
    p.stroke_rect(M + 4.0, p.y, CW - 4.0, h, CARD_LINE);
    let mut ty = p.y + 7.0;
    for line in &wrapped {
        p.text(M + 14.0, ty, "F3", 8.3, INK_SOFT, line);
        ty += 11.5;
    }
    p.y += h + 8.0;
}

fn recommendations(result: &ScanResult) -> Vec<String> {
    let mut recs = Vec::new();
    if !result.sql_vulnerable.is_empty() {
        recs.push("Use parameterised queries / prepared statements for every database call and validate all user input server-side.".into());
    }
    if !result.xss_vulnerable.is_empty() {
        recs.push("Escape HTML output, sanitise user-supplied content and deploy a strict Content-Security-Policy header.".into());
    }
    if !result.subdomain_takeovers.is_empty() {
        recs.push("Remove dangling DNS CNAME records and reclaim or delete the vulnerable subdomains immediately.".into());
    }
    if !result.zone_transfers.is_empty() {
        recs.push("Restrict DNS zone transfers to trusted secondary nameservers (AXFR should fail for untrusted hosts).".into());
    }
    if !result.cve_matches.is_empty() {
        recs.push("Upgrade the matched software components to the latest vendor-supported releases to clear known CVEs.".into());
    }
    if result.security_headers.contains("No important security headers") || result.security_headers.contains("Missing") {
        recs.push("Add missing HTTP security headers: HSTS, Content-Security-Policy, X-Frame-Options and X-Content-Type-Options.".into());
    }
    if !result.open_ports.is_empty() {
        recs.push("Review all open ports and close or firewall any service that is not required for business operation.".into());
    }
    if !result.discovered_paths.is_empty() {
        recs.push("Remove or protect exposed admin panels, backups and config files discovered in the path scan.".into());
    }
    recs.push("Re-run htool after remediation and keep the HTML/JSON/PDF reports for compliance tracking.".into());
    recs
}

fn stat_grid(p: &mut Pen, result: &ScanResult) {
    let stats: [(usize, &str, Rgb); 9] = [
        (result.open_ports.len(), "OPEN PORTS", EMERALD),
        (result.sql_vulnerable.len(), "SQL INJECTION", RED),
        (result.xss_vulnerable.len(), "XSS", RED),
        (result.subdomain_takeovers.len(), "TAKEOVERS", RED),
        (result.zone_transfers.len(), "ZONE TRANSFERS", ORANGE),
        (result.cve_matches.len(), "CVE MATCHES", AMBER),
        (result.discovered_paths.len(), "PATHS FOUND", CYAN),
        (result.subdomains.len(), "SUBDOMAINS", CYAN),
        (result.technologies.len(), "TECHNOLOGIES", PURPLE),
    ];
    let cols = 3.0_f32;
    let gap = 6.0;
    let cw = (CW - gap * (cols - 1.0)) / cols;
    let ch = 44.0;
    for (i, (num, label, color)) in stats.iter().enumerate() {
        let col = i % 3;
        if col == 0 { p.ensure(ch + gap); }
        let x = M + col as f32 * (cw + gap);
        p.rect(x, p.y, cw, ch, CARD_BG);
        p.stroke_rect(x, p.y, cw, ch, CARD_LINE);
        p.rect(x, p.y, cw, 2.2, *color);
        p.text(x + 10.0, p.y + 12.0, "F2", 15.0, *color, &num.to_string());
        p.text(x + 10.0, p.y + 31.0, "F1", 6.8, INK_FAINT, label);
        if col == 2 { p.y += ch + gap; }
    }
    if !stats.len().is_multiple_of(3) { p.y += ch + gap; }
}

/// Render the full report and return the PDF bytes.
pub fn generate_pdf_report(result: &ScanResult) -> Vec<u8> {
    let sev = result.severity();
    let sev_c = sev_rgb(sev);
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let mut p = Pen::new();

    // ── Header banner (full-bleed) ──
    p.rect(0.0, 0.0, PAGE_W, 118.0, NAVY);
    p.rect(0.0, 116.0, PAGE_W, 2.5, EMERALD);
    p.rect(M, 26.0, 4.0, 26.0, EMERALD);
    p.text(M + 14.0, 34.0, "F2", 23.0, WHITE, "SCAN REPORT");
    // severity pill
    let pill_text = format!("{} SEVERITY", sev);
    let pill_w = text_width(&pill_text, 9.0, true) + 22.0;
    let pill_x = M + 14.0 + text_width("SCAN REPORT", 23.0, true) + 18.0;
    p.rect(pill_x, 33.0, pill_w, 18.0, sev_c);
    p.text(pill_x + 11.0, 45.5, "F2", 9.0, WHITE, &pill_text);
    p.text(M + 14.0, 66.0, "F1", 10.5, (0.62, 0.68, 0.78), &pdf_escape(&result.target));
    p.text(M + 14.0, 86.0, "F1", 8.0, (0.42, 0.47, 0.58), &format!(
        "generated {}  ·  htool v{}  ·  authorised security assessment  ·  by Resolute Femi",
        now, env!("CARGO_PKG_VERSION")
    ));
    p.y = 140.0;

    // ── Severity summary line ──
    p.para(M, CW, "F2", 10.5, sev_c,
        &format!("Overall severity: {} — {} security findings · {} open ports · automated authorised scan.",
            sev, result.total_findings(), result.open_ports.len()),
        14.0);
    p.y += 6.0;

    // ── Stat grid ──
    stat_grid(&mut p, result);
    p.y += 4.0;

    // ── Findings sections ──
    section_header(&mut p, "Open Ports", result.open_ports.len(), EMERALD);
    let port_items: Vec<String> = result.open_ports.iter().map(|(prt, s)| format!("{} — {}", prt, s)).collect();
    bullet_list(&mut p, &port_items, EMERALD, "no open ports detected in scan range");

    section_header(&mut p, "SQL Injection", result.sql_vulnerable.len(), RED);
    bullet_list(&mut p, &result.sql_vulnerable, RED, "none found");

    section_header(&mut p, "Cross-Site Scripting (XSS)", result.xss_vulnerable.len(), RED);
    bullet_list(&mut p, &result.xss_vulnerable, ORANGE, "none found");

    if !result.subdomain_takeovers.is_empty() {
        section_header(&mut p, "Subdomain Takeovers", result.subdomain_takeovers.len(), RED);
        bullet_list(&mut p, &result.subdomain_takeovers, RED, "none");
    }
    if !result.zone_transfers.is_empty() {
        section_header(&mut p, "DNS Zone Transfers", result.zone_transfers.len(), RED);
        bullet_list(&mut p, &result.zone_transfers, RED, "none");
    }
    if !result.cve_matches.is_empty() {
        section_header(&mut p, "Known CVE Matches", result.cve_matches.len(), AMBER);
        bullet_list(&mut p, &result.cve_matches, AMBER, "none");
    }

    section_header(&mut p, "Discovered Paths", result.discovered_paths.len(), CYAN);
    bullet_list(&mut p, &result.discovered_paths, CYAN, "no exposed paths found");

    if !result.subdomains.is_empty() {
        section_header(&mut p, "Subdomains", result.subdomains.len(), CYAN);
        bullet_list(&mut p, &result.subdomains, CYAN, "none");
    }
    if !result.technologies.is_empty() {
        section_header(&mut p, "Detected Technologies", result.technologies.len(), PURPLE);
        bullet_list(&mut p, &result.technologies, PURPLE, "none");
    }

    section_header(&mut p, "SSL / TLS", 0, EMERALD);
    mono_block(&mut p, &result.ssl_info, 14);

    section_header(&mut p, "Security Headers", 0, EMERALD);
    mono_block(&mut p, &result.security_headers, 14);

    if !result.errors.is_empty() {
        section_header(&mut p, "Scan Errors", result.errors.len(), INK_FAINT);
        bullet_list(&mut p, &result.errors, INK_FAINT, "none");
    }

    // ── Recommendations ──
    p.ensure(40.0);
    p.hline(M, p.y + 4.0, CW, CARD_LINE);
    p.y += 16.0;
    section_header(&mut p, "Recommendations", recommendations(result).len(), EMERALD);
    let recs = recommendations(result);
    for (i, rec) in recs.iter().enumerate() {
        p.para(M + 4.0, CW - 18.0, "F1", 9.5, INK_SOFT, &format!("{}.  {}", i + 1, rec), 13.5);
    }

    // ── Finalise pages: append footer ops to every page ──
    p.finish_page();
    let total = p.pages.len();
    let mut page_streams: Vec<String> = Vec::new();
    for (idx, page_ops) in p.pages.iter().enumerate() {
        let mut ops = String::new();
        ops.push_str("0.85 0.88 0.93 RG 0.8 w\n");
        ops.push_str(&format!("{} {} m {} {} l S\n", f(M), f(34.0), f(PAGE_W - M), f(34.0)));
        ops.push_str(&format!("BT /F1 7.5 Tf {} {} Td 0.55 0.59 0.67 rg (Generated by htool v{} - Ultimate Hacker Toolkit, authorised use only) Tj ET\n",
            f(M), f(22.0), env!("CARGO_PKG_VERSION")));
        ops.push_str(&format!("BT /F1 7.5 Tf {} {} Td 0.55 0.59 0.67 rg (Page {} of {}) Tj ET\n",
            f(PAGE_W - M - 52.0), f(22.0), idx + 1, total));
        for line in page_ops {
            ops.push_str(line);
            ops.push('\n');
        }
        page_streams.push(ops);
    }

    assemble_pdf(&page_streams)
}

fn assemble_pdf(page_streams: &[String]) -> Vec<u8> {
    let n_pages = page_streams.len();
    // Object numbering:
    // 1: catalog, 2: pages, 3..=6: fonts (F1..F4), then per page: info obj + content obj
    let first_page_obj = 7usize;
    let mut objs: Vec<String> = Vec::new();

    objs.push("<< /Type /Catalog /Pages 2 0 R >>".into());
    let kids: Vec<String> = (0..n_pages).map(|i| format!("{} 0 R", first_page_obj + i * 2)).collect();
    objs.push(format!("<< /Type /Pages /Kids [{}] /Count {} >>", kids.join(" "), n_pages));
    objs.push("<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica /Encoding /WinAnsiEncoding >>".into());
    objs.push("<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica-Bold /Encoding /WinAnsiEncoding >>".into());
    objs.push("<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica-Oblique /Encoding /WinAnsiEncoding >>".into());
    objs.push("<< /Type /Font /Subtype /Type1 /BaseFont /Courier /Encoding /WinAnsiEncoding >>".into());

    for (i, stream) in page_streams.iter().enumerate() {
        let content_obj = first_page_obj + i * 2 + 1;
        objs.push(format!(
            "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {} {}] /Resources << /Font << /F1 3 0 R /F2 4 0 R /F3 5 0 R /F4 6 0 R >> >> /Contents {} 0 R >>",
            f(PAGE_W), f(PAGE_H), content_obj
        ));
        objs.push(format!("<< /Length {} >>\nstream\n{}\nendstream", latin1_len(stream), stream));
    }

    // Serialise with xref (byte-accurate: content encoded to WinAnsi/Latin-1)
    let mut out: Vec<u8> = Vec::new();
    out.extend_from_slice(b"%PDF-1.4\n%\xe2\xe3\xcf\xd3\n");
    let mut offsets = Vec::new();
    for (i, obj) in objs.iter().enumerate() {
        offsets.push(out.len());
        out.extend_from_slice(format!("{} 0 obj\n", i + 1).as_bytes());
        if i >= first_page_obj - 1 && (i - (first_page_obj - 1)) % 2 == 1 {
            // content stream object: payload must be Latin-1 encoded
            let header = format!("<< /Length {} >>\nstream\n", latin1_len(&page_streams[(i - (first_page_obj - 1) - 1) / 2]));
            out.extend_from_slice(header.as_bytes());
            out.extend(latin1(&page_streams[(i - (first_page_obj - 1) - 1) / 2]));
            out.extend_from_slice(b"\nendstream");
        } else {
            out.extend_from_slice(obj.as_bytes());
        }
        out.extend_from_slice(b"\nendobj\n");
    }
    let xref_pos = out.len();
    out.extend_from_slice(format!("xref\n0 {}\n", objs.len() + 1).as_bytes());
    out.extend_from_slice(b"0000000000 65535 f \n");
    for off in &offsets {
        out.extend_from_slice(format!("{:010} 00000 n \n", off).as_bytes());
    }
    out.extend_from_slice(format!("trailer\n<< /Size {} /Root 1 0 R >>\nstartxref\n{}\n%%EOF\n", objs.len() + 1, xref_pos).as_bytes());
    out
}

/// Encode a string to Latin-1/WinAnsi bytes (string payloads in this PDF are
/// pre-sanitised by `pdf_escape`, so all remaining chars are <= U+00FF).
fn latin1(s: &str) -> Vec<u8> {
    s.chars().map(|c| if (c as u32) < 0x100 { c as u8 } else { b'?' }).collect()
}

fn latin1_len(s: &str) -> usize {
    s.chars().count()
}

/// Write the PDF report to a file.
pub fn export_pdf_report(result: &ScanResult, path: &str) -> Result<(), String> {
    let bytes = generate_pdf_report(result);
    std::fs::write(path, bytes).map_err(|e| format!("Failed to write PDF report: {}", e))
}
