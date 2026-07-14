// ─── SVG Icon Components for htool Feature Cards ─────────
// Cyberpunk-styled SVG icons replacing emoji placeholders
// ─────────────────────────────────────────────────────────

export function IconScanner({ className = "w-8 h-8" }) {
  return (
    <svg className={className} viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
      {/* Magnifying glass / Scanner */}
      <circle cx="20" cy="20" r="12" stroke="#00ff41" strokeWidth="2.5" fill="none" />
      <line x1="29" y1="29" x2="40" y2="40" stroke="#0aefff" strokeWidth="3" strokeLinecap="round" />
      {/* Radar arcs */}
      <path d="M20 8 A12 12 0 0 1 32 20" stroke="#00ff41" strokeWidth="1.5" strokeDasharray="3 3" fill="none" opacity="0.6" />
      <circle cx="20" cy="20" r="4" fill="#00ff41" opacity="0.8" />
      {/* Crosshair ticks */}
      <line x1="20" y1="4" x2="20" y2="10" stroke="#00ff41" strokeWidth="1.5" opacity="0.4" />
      <line x1="20" y1="30" x2="20" y2="36" stroke="#00ff41" strokeWidth="1.5" opacity="0.4" />
      <line x1="4" y1="20" x2="10" y2="20" stroke="#00ff41" strokeWidth="1.5" opacity="0.4" />
      <line x1="30" y1="20" x2="36" y2="20" stroke="#00ff41" strokeWidth="1.5" opacity="0.4" />
    </svg>
  )
}

export function IconStress({ className = "w-8 h-8" }) {
  return (
    <svg className={className} viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
      {/* Lightning bolt / Stress */}
      <polygon points="26,4 10,26 22,26 18,44 38,20 26,20 30,4" fill="#00ff41" opacity="0.9" />
      {/* Energy arcs */}
      <path d="M4 12 Q12 8 8 16" stroke="#0aefff" strokeWidth="1.5" fill="none" opacity="0.6" />
      <path d="M44 32 Q36 36 40 28" stroke="#0aefff" strokeWidth="1.5" fill="none" opacity="0.6" />
      {/* Impact circles */}
      <circle cx="38" cy="16" r="2" fill="#ff0055" opacity="0.7" />
      <circle cx="40" cy="12" r="1.5" fill="#ff0055" opacity="0.5" />
      <circle cx="42" cy="8" r="1" fill="#ff0055" opacity="0.3" />
    </svg>
  )
}

export function IconCredential({ className = "w-8 h-8" }) {
  return (
    <svg className={className} viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
      {/* Key icon */}
      <circle cx="18" cy="26" r="12" stroke="#00ff41" strokeWidth="2.5" fill="none" />
      <circle cx="18" cy="26" r="5" stroke="#0aefff" strokeWidth="2" fill="none" />
      <line x1="26" y1="34" x2="38" y2="46" stroke="#00ff41" strokeWidth="2.5" strokeLinecap="round" />
      <line x1="32" y1="40" x2="38" y2="46" stroke="#00ff41" strokeWidth="2.5" strokeLinecap="round" />
      <line x1="30" y1="36" x2="36" y2="42" stroke="#00ff41" strokeWidth="2" strokeLinecap="round" opacity="0.5" />
      {/* Key teeth */}
      <line x1="36" y1="42" x2="40" y2="40" stroke="#0aefff" strokeWidth="2" strokeLinecap="round" />
      <line x1="38" y1="44" x2="42" y2="42" stroke="#0aefff" strokeWidth="2" strokeLinecap="round" />
    </svg>
  )
}

export function IconSpam({ className = "w-8 h-8" }) {
  return (
    <svg className={className} viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
      {/* Email/Spam icon */}
      <rect x="4" y="10" width="40" height="28" rx="4" stroke="#00ff41" strokeWidth="2" fill="none" />
      <polyline points="4,16 24,28 44,16" stroke="#0aefff" strokeWidth="2" fill="none" />
      {/* Flood lines */}
      <line x1="14" y1="22" x2="20" y2="26" stroke="#ff0055" strokeWidth="1.5" opacity="0.6" />
      <line x1="28" y1="22" x2="34" y2="26" stroke="#ff0055" strokeWidth="1.5" opacity="0.6" />
      {/* Multiple dots (flood) */}
      <circle cx="10" cy="34" r="1.5" fill="#00ff41" opacity="0.5" />
      <circle cx="16" cy="36" r="1.5" fill="#00ff41" opacity="0.5" />
      <circle cx="22" cy="34" r="1.5" fill="#0aefff" opacity="0.5" />
      <circle cx="28" cy="36" r="1.5" fill="#00ff41" opacity="0.5" />
      <circle cx="34" cy="34" r="1.5" fill="#0aefff" opacity="0.5" />
      <circle cx="38" cy="36" r="1.5" fill="#00ff41" opacity="0.5" />
    </svg>
  )
}

export function IconPayload({ className = "w-8 h-8" }) {
  return (
    <svg className={className} viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
      {/* Terminal / Payload */}
      <rect x="4" y="8" width="40" height="32" rx="3" stroke="#00ff41" strokeWidth="2" fill="none" />
      {/* Terminal dots */}
      <circle cx="12" cy="16" r="2" fill="#ff0055" />
      <circle cx="20" cy="16" r="2" fill="#ffa500" />
      <circle cx="28" cy="16" r="2" fill="#00ff41" />
      {/* Code lines */}
      <line x1="10" y1="26" x2="22" y2="26" stroke="#0aefff" strokeWidth="2" strokeLinecap="round" />
      <line x1="10" y1="32" x2="30" y2="32" stroke="#00ff41" strokeWidth="2" strokeLinecap="round" />
      {/* Blinking cursor */}
      <line x1="32" y1="30" x2="32" y2="34" stroke="#00ff41" strokeWidth="1.5" opacity="0.7" />
      {/* Brackets */}
      <text x="34" y="28" fill="#ff0055" fontSize="10" fontFamily="monospace" fontWeight="bold">&gt;_</text>
    </svg>
  )
}

export function IconReport({ className = "w-8 h-8" }) {
  return (
    <svg className={className} viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
      {/* Document / Report */}
      <path d="M8 4 H30 L40 14 V44 H8 Z" stroke="#00ff41" strokeWidth="2" fill="none" />
      <path d="M30 4 V14 H40" stroke="#0aefff" strokeWidth="2" fill="none" opacity="0.7" />
      {/* Content lines */}
      <line x1="14" y1="20" x2="34" y2="20" stroke="#00ff41" strokeWidth="1.5" strokeLinecap="round" opacity="0.6" />
      <line x1="14" y1="26" x2="30" y2="26" stroke="#0aefff" strokeWidth="1.5" strokeLinecap="round" opacity="0.5" />
      <line x1="14" y1="32" x2="34" y2="32" stroke="#00ff41" strokeWidth="1.5" strokeLinecap="round" opacity="0.4" />
      <line x1="14" y1="38" x2="26" y2="38" stroke="#0aefff" strokeWidth="1.5" strokeLinecap="round" opacity="0.3" />
      {/* Chart bar */}
      <rect x="32" y="24" width="3" height="10" fill="#00ff41" opacity="0.8" rx="1" />
      <rect x="26" y="28" width="3" height="6" fill="#0aefff" opacity="0.6" rx="1" />
    </svg>
  )
}

export function IconCve({ className = "w-8 h-8" }) {
  return (
    <svg className={className} viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
      {/* Database / CVE */}
      <ellipse cx="24" cy="12" rx="18" ry="6" stroke="#00ff41" strokeWidth="2" fill="none" />
      <path d="M6 12 V36 C6 39.3 14.1 42 24 42 C33.9 42 42 39.3 42 36 V12" stroke="#00ff41" strokeWidth="2" fill="none" />
      <path d="M6 24 C6 27.3 14.1 30 24 30 C33.9 30 42 27.3 42 24" stroke="#0aefff" strokeWidth="1.5" fill="none" opacity="0.6" />
      {/* Rows */}
      <line x1="14" y1="19" x2="20" y2="19" stroke="#00ff41" strokeWidth="1.5" opacity="0.5" />
      <line x1="14" y1="28" x2="22" y2="28" stroke="#0aefff" strokeWidth="1.5" opacity="0.5" />
      <line x1="14" y1="33" x2="18" y2="33" stroke="#00ff41" strokeWidth="1.5" opacity="0.5" />
      <line x1="30" y1="18" x2="34" y2="18" stroke="#ff0055" strokeWidth="1.5" opacity="0.7" />
      <line x1="30" y1="27" x2="32" y2="27" stroke="#ff0055" strokeWidth="1.5" opacity="0.7" />
    </svg>
  )
}

export function IconHtmlViewer({ className = "w-8 h-8" }) {
  return (
    <svg className={className} viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
      {/* HTML / Viewer */}
      <rect x="4" y="8" width="40" height="32" rx="3" stroke="#00ff41" strokeWidth="2" fill="none" />
      {/* Tag brackets */}
      <path d="M12 20 L8 24 L12 28" stroke="#0aefff" strokeWidth="2" fill="none" strokeLinecap="round" strokeLinejoin="round" />
      <path d="M36 20 L40 24 L36 28" stroke="#0aefff" strokeWidth="2" fill="none" strokeLinecap="round" strokeLinejoin="round" />
      {/* Eye / viewer */}
      <path d="M18 24 Q24 18 30 24 Q24 30 18 24 Z" stroke="#00ff41" strokeWidth="1.5" fill="none" />
      <circle cx="24" cy="24" r="2.5" fill="#00ff41" opacity="0.8" />
      {/* Code brackets */}
      <text x="15" y="16" fill="#ff0055" fontSize="8" fontFamily="monospace">&lt;/&gt;</text>
    </svg>
  )
}

export function IconWaf({ className = "w-8 h-8" }) {
  return (
    <svg className={className} viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
      {/* Shield / WAF */}
      <path d="M24 4 L44 14 V26 C44 38 24 46 24 46 C24 46 4 38 4 26 V14 Z" stroke="#00ff41" strokeWidth="2" fill="none" />
      <path d="M24 4 L44 14 V26 C44 38 24 46 24 46 C24 46 4 38 4 26 V14 Z" stroke="#0aefff" strokeWidth="1" fill="none" opacity="0.3" transform="translate(2,2)" />
      {/* Checkmark inside shield */}
      <polyline points="18,24 23,30 32,18" stroke="#00ff41" strokeWidth="2.5" fill="none" strokeLinecap="round" strokeLinejoin="round" />
      {/* Detection waves */}
      <path d="M8 20 Q12 16 16 20" stroke="#ff0055" strokeWidth="1" fill="none" opacity="0.5" />
      <path d="M32 20 Q36 16 40 20" stroke="#ff0055" strokeWidth="1" fill="none" opacity="0.5" />
      <path d="M12 14 Q16 10 20 14" stroke="#ffa500" strokeWidth="1" fill="none" opacity="0.4" />
      <path d="M28 14 Q32 10 36 14" stroke="#ffa500" strokeWidth="1" fill="none" opacity="0.4" />
    </svg>
  )
}

// ─── Lion App Icon (Favicon / App Icon) ───────────────────
export function LionIcon({ className = "w-8 h-8" }) {
  return (
    <svg className={className} viewBox="0 0 100 100" fill="none" xmlns="http://www.w3.org/2000/svg">
      {/* Background circle */}
      <circle cx="50" cy="50" r="48" fill="#080c16" stroke="#00ff41" strokeWidth="3" />
      <circle cx="50" cy="50" r="45" stroke="#0aefff" strokeWidth="0.5" opacity="0.3" />

      {/* Lion mane - outer ring */}
      <g stroke="#00ff41" strokeWidth="1.5" fill="none" opacity="0.6">
        <line x1="50" y1="12" x2="50" y2="18" />
        <line x1="72" y1="18" x2="68" y2="23" />
        <line x1="86" y1="36" x2="80" y2="38" />
        <line x1="88" y1="56" x2="82" y2="54" />
        <line x1="78" y1="74" x2="73" y2="70" />
        <line x1="62" y1="84" x2="60" y2="78" />
        <line x1="38" y1="84" x2="40" y2="78" />
        <line x1="22" y1="74" x2="27" y2="70" />
        <line x1="12" y1="56" x2="18" y2="54" />
        <line x1="14" y1="36" x2="20" y2="38" />
        <line x1="28" y1="18" x2="32" y2="23" />
      </g>

      {/* Additional mane triangles */}
      <g fill="#00ff41" opacity="0.15">
        <polygon points="50,10 46,18 54,18" />
        <polygon points="76,22 70,28 78,26" />
        <polygon points="88,42 82,44 86,50" />
        <polygon points="88,62 82,60 84,68" />
        <polygon points="72,80 68,74 76,76" />
        <polygon points="52,90 50,84 58,86" />
        <polygon points="28,80 24,74 32,76" />
        <polygon points="12,62 16,60 18,68" />
        <polygon points="14,42 18,44 20,38" />
        <polygon points="30,22 32,28 36,24" />
      </g>

      {/* Inner glow */}
      <circle cx="50" cy="50" r="28" stroke="#0aefff" strokeWidth="0.5" opacity="0.2" fill="none" />

      {/* Lion face - circle */}
      <circle cx="50" cy="48" r="22" fill="#0c1220" stroke="#0aefff" strokeWidth="1.5" opacity="0.8" />

      {/* Ears */}
      <ellipse cx="32" cy="32" rx="7" ry="5" stroke="#00ff41" strokeWidth="1.5" fill="none" opacity="0.7" />
      <ellipse cx="68" cy="32" rx="7" ry="5" stroke="#00ff41" strokeWidth="1.5" fill="none" opacity="0.7" />
      <circle cx="32" cy="32" r="3" fill="#0aefff" opacity="0.3" />
      <circle cx="68" cy="32" r="3" fill="#0aefff" opacity="0.3" />

      {/* Eyes - cyberpunk glowing */}
      <ellipse cx="39" cy="44" rx="5" ry="4" fill="none" stroke="#0aefff" strokeWidth="1.5" />
      <ellipse cx="61" cy="44" rx="5" ry="4" fill="none" stroke="#0aefff" strokeWidth="1.5" />
      <circle cx="39" cy="44" r="2.5" fill="#0aefff" opacity="0.9" />
      <circle cx="61" cy="44" r="2.5" fill="#0aefff" opacity="0.9" />
      {/* Eye inner glow */}
      <circle cx="39" cy="44" r="1" fill="#00ff41" />
      <circle cx="61" cy="44" r="1" fill="#00ff41" />

      {/* Nose */}
      <polygon points="50,50 46,54 54,54" fill="#00ff41" opacity="0.8" stroke="#0aefff" strokeWidth="0.5" />

      {/* Mouth */}
      <path d="M44 56 Q50 62 56 56" stroke="#00ff41" strokeWidth="1.5" fill="none" strokeLinecap="round" />
      <line x1="50" y1="54" x2="50" y2="58" stroke="#00ff41" strokeWidth="1" />

      {/* Whiskers */}
      <line x1="18" y1="48" x2="32" y2="50" stroke="#0aefff" strokeWidth="1" opacity="0.4" />
      <line x1="18" y1="52" x2="32" y2="52" stroke="#0aefff" strokeWidth="1" opacity="0.4" />
      <line x1="82" y1="48" x2="68" y2="50" stroke="#0aefff" strokeWidth="1" opacity="0.4" />
      <line x1="82" y1="52" x2="68" y2="52" stroke="#0aefff" strokeWidth="1" opacity="0.4" />

      {/* Tech/Matrix details on forehead */}
      <text x="47" y="36" fill="#00ff41" fontSize="6" fontFamily="monospace" opacity="0.6" letterSpacing="1">01</text>
      <circle cx="45" cy="30" r="1" fill="#00ff41" opacity="0.4" />
      <circle cx="55" cy="30" r="1" fill="#00ff41" opacity="0.4" />

      {/* Cyberpunk accent marks */}
      <line x1="72" y1="48" x2="78" y2="48" stroke="#ff0055" strokeWidth="1" opacity="0.5" />
      <line x1="22" y1="48" x2="28" y2="48" stroke="#ff0055" strokeWidth="1" opacity="0.5" />
    </svg>
  )
}


