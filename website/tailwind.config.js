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
        // matrix-500 is a CSS var so light mode can shift to a deeper green
        matrix: {
          50: '#e6fff8',
          100: '#b3ffed',
          200: '#80ffdf',
          300: '#4dffce',
          400: 'rgb(var(--m-400) / <alpha-value>)',
          500: 'rgb(var(--m-500) / <alpha-value>)',
          600: '#00b87e',
          700: '#00895e',
          800: '#005c3f',
          900: '#002e1f',
        },
        cyber: {
          cyan: 'rgb(var(--c-cyan) / <alpha-value>)',
          pink: '#f472b6',
          orange: 'rgb(var(--c-orange) / <alpha-value>)',
          purple: 'rgb(var(--c-purple) / <alpha-value>)',
          dark: 'rgb(var(--c-dark) / <alpha-value>)',
          deep: 'rgb(var(--c-deep) / <alpha-value>)',
          card: 'rgb(var(--c-card) / <alpha-value>)',
          row: 'rgb(var(--c-row) / <alpha-value>)',
          sidebar: 'rgb(var(--c-sidebar) / <alpha-value>)',
          dim: 'rgb(var(--c-dim) / <alpha-value>)',
          faint: 'rgb(var(--c-faint) / <alpha-value>)',
          bright: 'rgb(var(--c-bright) / <alpha-value>)',
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
