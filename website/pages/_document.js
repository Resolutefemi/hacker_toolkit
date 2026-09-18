import { Html, Head, Main, NextScript } from 'next/document'

// Static-export base path (set NEXT_PUBLIC_BASE_PATH when building for GitHub Pages)
const bp = process.env.NEXT_PUBLIC_BASE_PATH || ''
const SITE_URL = process.env.NEXT_PUBLIC_SITE_URL || `https://resolutefemi.github.io${bp}`

export default function Document() {
  return (
    <Html lang="en">
      <Head>
        {/* ─── Apply saved theme before first paint (no flash) ─── */}
        <script
          dangerouslySetInnerHTML={{
            __html: "try{var t=localStorage.getItem('htool-theme');if(t==='light'||(t===null&&window.matchMedia('(prefers-color-scheme: light)').matches)){document.documentElement.classList.add('light')}}catch(e){}"
          }}
        />
        {/* ─── Primary Meta Tags ─── */}
        <meta charSet="utf-8" />
        <meta name="title" content="htool - Ultimate Cybersecurity Toolkit by Resolute Femi" />
        <meta name="description" content="Download htool by Ariyo Oluwafemi Stephen (Resolute Femi), the ultimate all-in-one cybersecurity testing toolkit. Features vulnerability scanning, stress testing, credential testing, payload generation, and more. Advanced tools for checking vulnerability." />
        <meta name="keywords" content="htool, Resolute Femi, Resolutefemi, Ariyo Oluwafemi Stephen, tools for checking vulnerability, cybersecurity toolkit, vulnerability scanner, penetration testing, security tools" />
        <meta name="author" content="Ariyo Oluwafemi Stephen (Resolute Femi)" />
        <meta name="robots" content="index, follow" />
        <meta name="revisit-after" content="7 days" />

        {/* ─── Open Graph / Facebook ─── */}
        <meta property="og:type" content="website" />
        <meta property="og:url" content={`${SITE_URL}/`} />
        <meta property="og:title" content="htool - Cybersecurity Toolkit by Resolute Femi" />
        <meta property="og:description" content="Download htool, the ultimate all-in-one cybersecurity testing toolkit. Vulnerability scanning, stress testing, credential testing, payload generation, and more." />
        <meta property="og:image" content={`${bp}/og-image.png`} />

        {/* ─── Twitter ─── */}
        <meta property="twitter:card" content="summary_large_image" />
        <meta property="twitter:url" content={`${SITE_URL}/`} />
        <meta property="twitter:title" content="htool - Cybersecurity Toolkit by Resolute Femi" />
        <meta property="twitter:description" content="Download htool by Ariyo Oluwafemi Stephen (Resolute Femi), the ultimate cybersecurity testing toolkit. Tools for checking vulnerability." />
        <meta property="twitter:image" content={`${bp}/og-image.png`} />

        {/* ─── Canonical ─── */}
        <link rel="canonical" href={`${SITE_URL}/`} />

        {/* ─── Favicon ─── */}
        <link rel="icon" type="image/svg+xml" href={`${bp}/favicon.svg`} />
        <link rel="apple-touch-icon" sizes="180x180" href={`${bp}/apple-touch-icon.png`} />

        {/* ─── Structured Data (JSON-LD) ─── */}
        <script
          type="application/ld+json"
          dangerouslySetInnerHTML={{
            __html: JSON.stringify({
              "@context": "https://schema.org",
              "@type": "SoftwareApplication",
              "name": "htool",
              "operatingSystem": "Windows, macOS, Linux",
              "applicationCategory": "SecurityApplication",
              "author": {
                "@type": "Person",
                "name": "Ariyo Oluwafemi Stephen",
                "alternateName": "Resolute Femi"
              },
              "description": "An all-in-one cybersecurity testing framework for vulnerability scanning, stress testing, credential stuffing, spam testing, payload generation, and reporting.",
              "offers": {
                "@type": "Offer",
                "price": "0",
                "priceCurrency": "USD"
              },
              "keywords": "htool, Resolute Femi, Resolutefemi, tools for checking vulnerability, cybersecurity"
            })
          }}
        />

        {/* ─── Fonts ─── */}
        <link rel="preconnect" href="https://fonts.googleapis.com" />
        <link rel="preconnect" href="https://fonts.gstatic.com" crossOrigin="true" />
      </Head>
      <body>
        <Main />
        <NextScript />
      </body>
    </Html>
  )
}
