/** @type {import('next').NextConfig} */

// Static-export base path — set NEXT_PUBLIC_BASE_PATH=/hacker_toolkit when
// building for GitHub Pages project sites. Leave empty for root-domain hosts
// (e.g. Cloudflare Pages *.pages.dev).
const basePath = process.env.NEXT_PUBLIC_BASE_PATH || ''

const nextConfig = {
  reactStrictMode: true,
  output: 'export',
  basePath,
  assetPrefix: basePath || undefined,
  images: {
    unoptimized: true,
  },
}

module.exports = nextConfig
