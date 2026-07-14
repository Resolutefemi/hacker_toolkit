/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./pages/**/*.{js,ts,jsx,tsx,mdx}",
    "./components/**/*.{js,ts,jsx,tsx,mdx}",
  ],
  theme: {
    extend: {
      colors: {
        matrix: {
          50: '#e6ffe6',
          100: '#b3ffb3',
          200: '#80ff80',
          300: '#4dff4d',
          400: '#1aff1a',
          500: '#00ff41',
          600: '#00cc34',
          700: '#009927',
          700: '#00661a',
          900: '#00330d',
        },
        cyber: {
          cyan: '#0aefff',
          pink: '#ff0055',
          orange: '#ffa500',
          dark: '#080c16',
          card: '#0c1220',
          sidebar: '#060a12',
          dim: '#64788c',
          bright: '#c8d7e6',
        },
      },
      fontFamily: {
        mono: ['JetBrains Mono', 'Fira Code', 'monospace'],
        sans: ['Inter', 'system-ui', 'sans-serif'],
      },
      animation: {
        'glow': 'glow 2s ease-in-out infinite alternate',
        'pulse-slow': 'pulse 3s cubic-bezier(0.4, 0, 0.6, 1) infinite',
        'scan-line': 'scan 3s linear infinite',
      },
      keyframes: {
        glow: {
          '0%': { textShadow: '0 0 10px #00ff41, 0 0 20px #00ff41' },
          '100%': { textShadow: '0 0 20px #00ff41, 0 0 40px #0aefff, 0 0 80px #0aefff' },
        },
        scan: {
          '0%': { transform: 'translateY(-100%)' },
          '100%': { transform: 'translateY(100vh)' },
        },
      },
    },
  },
  plugins: [],
}
