// ─── SVG Icon Components for htool ────────────────────────
// Professional SVG icons for feature cards, OS logos, and branding
// ───────────────────────────────────────────────────────────

// ─── Feature Icons ────────────────────────────────────────

export function IconScanner({ className = "w-8 h-8" }) {
  return (
    <svg className={className} viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
      <circle cx="20" cy="20" r="12" stroke="#00ff41" strokeWidth="2.5" fill="none" />
      <line x1="29" y1="29" x2="40" y2="40" stroke="#0aefff" strokeWidth="3" strokeLinecap="round" />
      <path d="M20 8 A12 12 0 0 1 32 20" stroke="#00ff41" strokeWidth="1.5" strokeDasharray="3 3" fill="none" opacity="0.6" />
      <circle cx="20" cy="20" r="4" fill="#00ff41" opacity="0.8" />
    </svg>
  )
}

export function IconStress({ className = "w-8 h-8" }) {
  return (
    <svg className={className} viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
      <polygon points="26,4 10,26 22,26 18,44 38,20 26,20 30,4" fill="#00ff41" opacity="0.9" />
    </svg>
  )
}

export function IconCredential({ className = "w-8 h-8" }) {
  return (
    <svg className={className} viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
      <circle cx="18" cy="26" r="12" stroke="#00ff41" strokeWidth="2.5" fill="none" />
      <circle cx="18" cy="26" r="5" stroke="#0aefff" strokeWidth="2" fill="none" />
      <line x1="26" y1="34" x2="38" y2="46" stroke="#00ff41" strokeWidth="2.5" strokeLinecap="round" />
      <line x1="32" y1="40" x2="38" y2="46" stroke="#00ff41" strokeWidth="2.5" strokeLinecap="round" />
    </svg>
  )
}

export function IconSpam({ className = "w-8 h-8" }) {
  return (
    <svg className={className} viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
      <rect x="4" y="10" width="40" height="28" rx="4" stroke="#00ff41" strokeWidth="2" fill="none" />
      <polyline points="4,16 24,28 44,16" stroke="#0aefff" strokeWidth="2" fill="none" />
    </svg>
  )
}

export function IconPayload({ className = "w-8 h-8" }) {
  return (
    <svg className={className} viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
      <rect x="4" y="8" width="40" height="32" rx="3" stroke="#00ff41" strokeWidth="2" fill="none" />
      <circle cx="12" cy="16" r="2" fill="#ff0055" />
      <circle cx="20" cy="16" r="2" fill="#ffa500" />
      <circle cx="28" cy="16" r="2" fill="#00ff41" />
      <line x1="10" y1="26" x2="22" y2="26" stroke="#0aefff" strokeWidth="2" strokeLinecap="round" />
      <line x1="10" y1="32" x2="30" y2="32" stroke="#00ff41" strokeWidth="2" strokeLinecap="round" />
    </svg>
  )
}

export function IconReport({ className = "w-8 h-8" }) {
  return (
    <svg className={className} viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
      <path d="M8 4 H30 L40 14 V44 H8 Z" stroke="#00ff41" strokeWidth="2" fill="none" />
      <path d="M30 4 V14 H40" stroke="#0aefff" strokeWidth="2" fill="none" opacity="0.7" />
      <line x1="14" y1="22" x2="34" y2="22" stroke="#00ff41" strokeWidth="1.5" strokeLinecap="round" opacity="0.5" />
      <line x1="14" y1="30" x2="30" y2="30" stroke="#0aefff" strokeWidth="1.5" strokeLinecap="round" opacity="0.4" />
    </svg>
  )
}

export function IconCve({ className = "w-8 h-8" }) {
  return (
    <svg className={className} viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
      <ellipse cx="24" cy="12" rx="18" ry="6" stroke="#00ff41" strokeWidth="2" fill="none" />
      <path d="M6 12 V36 C6 39.3 14.1 42 24 42 C33.9 42 42 39.3 42 36 V12" stroke="#00ff41" strokeWidth="2" fill="none" />
      <path d="M6 24 C6 27.3 14.1 30 24 30 C33.9 30 42 27.3 42 24" stroke="#0aefff" strokeWidth="1.5" fill="none" opacity="0.6" />
    </svg>
  )
}

export function IconHtmlViewer({ className = "w-8 h-8" }) {
  return (
    <svg className={className} viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
      <rect x="4" y="8" width="40" height="32" rx="3" stroke="#00ff41" strokeWidth="2" fill="none" />
      <path d="M12 20 L8 24 L12 28" stroke="#0aefff" strokeWidth="2" fill="none" strokeLinecap="round" strokeLinejoin="round" />
      <path d="M36 20 L40 24 L36 28" stroke="#0aefff" strokeWidth="2" fill="none" strokeLinecap="round" strokeLinejoin="round" />
      <path d="M18 24 Q24 18 30 24 Q24 30 18 24 Z" stroke="#00ff41" strokeWidth="1.5" fill="none" />
      <circle cx="24" cy="24" r="2.5" fill="#00ff41" opacity="0.8" />
    </svg>
  )
}

export function IconWaf({ className = "w-8 h-8" }) {
  return (
    <svg className={className} viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
      <path d="M24 4 L44 14 V26 C44 38 24 46 24 46 C24 46 4 38 4 26 V14 Z" stroke="#00ff41" strokeWidth="2" fill="none" />
      <polyline points="18,24 23,30 32,18" stroke="#00ff41" strokeWidth="2.5" fill="none" strokeLinecap="round" strokeLinejoin="round" />
    </svg>
  )
}

// ─── OS Logo Icons ────────────────────────────────────────

export function OsWindows({ className = "w-8 h-8" }) {
  return (
    <svg className={className} viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
      <rect x="4" y="4" width="18" height="18" rx="2" stroke="#00ff41" strokeWidth="2" fill="none" />
      <rect x="26" y="4" width="18" height="18" rx="2" stroke="#00ff41" strokeWidth="2" fill="none" />
      <rect x="4" y="26" width="18" height="18" rx="2" stroke="#00ff41" strokeWidth="2" fill="none" />
      <rect x="26" y="26" width="18" height="18" rx="2" stroke="#00ff41" strokeWidth="2" fill="none" />
      <line x1="13" y1="4" x2="13" y2="22" stroke="#0aefff" strokeWidth="1.5" opacity="0.5" />
      <line x1="4" y1="13" x2="22" y2="13" stroke="#0aefff" strokeWidth="1.5" opacity="0.5" />
    </svg>
  )
}

export function OsMacos({ className = "w-8 h-8" }) {
  return (
    <svg className={className} viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
      <path d="M36 24 C36 16 30 12 24 8 C18 12 12 16 12 24 C12 32 18 40 24 44 C30 40 36 32 36 24Z" stroke="#00ff41" strokeWidth="2" fill="none" />
      <path d="M24 8 C26 4 30 4 32 6" stroke="#0aefff" strokeWidth="1.5" fill="none" strokeLinecap="round" />
      <path d="M32 28 C34 26 34 22 32 20" stroke="#080c16" strokeWidth="3" fill="none" />
    </svg>
  )
}

export function OsLinux({ className = "w-8 h-8" }) {
  return (
    <svg className={className} viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
      <ellipse cx="24" cy="26" rx="14" ry="16" stroke="#00ff41" strokeWidth="2" fill="none" />
      <circle cx="24" cy="18" r="10" stroke="#00ff41" strokeWidth="2" fill="none" />
      <circle cx="20" cy="16" r="2.5" fill="#0aefff" opacity="0.8" />
      <circle cx="28" cy="16" r="2.5" fill="#0aefff" opacity="0.8" />
      <polygon points="22,20 26,20 24,23" fill="#ffa500" opacity="0.7" />
      <ellipse cx="24" cy="30" rx="8" ry="8" stroke="#0aefff" strokeWidth="1" fill="none" opacity="0.4" />
      <path d="M14 40 L10 44 M10 42 L18 42" stroke="#ffa500" strokeWidth="1.5" strokeLinecap="round" opacity="0.6" />
      <path d="M34 40 L38 44 M38 42 L30 42" stroke="#ffa500" strokeWidth="1.5" strokeLinecap="round" opacity="0.6" />
    </svg>
  )
}

// ─── Brand H Icon ─────────────────────────────────────────
// Simple green H with red underline for app branding
export function HIcon({ className = "w-8 h-8" }) {
  return (
    <svg className={className} viewBox="0 0 100 100" fill="none" xmlns="http://www.w3.org/2000/svg">
      {/* Dark background */}
      <rect x="4" y="4" width="92" height="92" rx="16" fill="#080c16" stroke="#00ff41" strokeWidth="2" />
      
      {/* Green H letter */}
      <text x="50" y="68" fontFamily="monospace" fontSize="60" fontWeight="bold" 
            fill="#00ff41" textAnchor="middle" letterSpacing="2">H</text>
      
      {/* Red underline */}
      <line x1="22" y1="78" x2="78" y2="78" stroke="#ff0055" strokeWidth="3.5" strokeLinecap="round"/>
    </svg>
  )
}
