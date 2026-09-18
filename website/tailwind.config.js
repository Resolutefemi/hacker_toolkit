/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./pages/**/*.{js,ts,jsx,tsx,mdx}",
    "./components/**/*.{js,ts,jsx,tsx,mdx}",
  ],
  theme: {
    extend: {
      colors: {
        // Primary accent — neon emerald (matches the new app UI)
        matrix: {
          50: '#e6fff8',
          100: '#b3ffed',
          200: '#80ffdf',
          300: '#4dffce',
          400: '#1affb9',
          500: '#00e69e',
          600: '#00b87e',
          700: '#00895e',
          800: '#005c3f',
          900: '#002e1f',
        },
        cyber: {
          cyan: '#38bdf8',
          pink: '#f472b6',
          orange: '#fb923c',
          purple: '#a78bfa',
          dark: '#05080f',
          deep: '#070b14',
          card: '#111729',
          row: '#0b1120',
          sidebar: '#080d18',
          dim: '#8c98b2',
          faint: '#606a82',
          bright: '#e2e8f0',
        },
      },
      fontFamily: {
        mono: ['"JetBrains Mono"', 'Fira Code', 'monospace'],
        sans: ['Inter', 'system-ui', 'sans-serif'],
        display: ['"Space Grotesk"', 'Inter', 'system-ui', 'sans-serif'],
      },
      animation: {
        'glow': 'glow 2.4s ease-in-out infinite alternate',
        'pulse-slow': 'pulse 3.2s cubic-bezier(0.4, 0, 0.6, 1) infinite',
        'float': 'float 7s ease-in-out infinite',
        'float-slow': 'float 11s ease-in-out infinite',
        'blink': 'blink 1.1s step-end infinite',
        'spin-slow': 'spin 14s linear infinite',
      },
      keyframes: {
        glow: {
          '0%': { textShadow: '0 0 8px rgba(0,230,158,0.5), 0 0 18px rgba(0,230,158,0.3)' },
          '100%': { textShadow: '0 0 16px rgba(0,230,158,0.7), 0 0 36px rgba(56,189,248,0.4)' },
        },
        float: {
          '0%, 100%': { transform: 'translateY(0px)' },
          '50%': { transform: 'translateY(-14px)' },
        },
        blink: {
          '0%, 100%': { opacity: '1' },
          '50%': { opacity: '0' },
        },
      },
      boxShadow: {
        'glow-accent': '0 0 28px rgba(0,230,158,0.28), 0 0 90px rgba(0,230,158,0.12)',
        'card': '0 8px 32px rgba(0,0,0,0.4)',
      },
    },
  },
  plugins: [],
}
