/** @type {import('next').NextConfig} */

module.exports = {
  reactStrictMode: true,

  swcMinify: true,

  images: {
    unoptimized: true,
  },

  // https://nextjs.org/docs/api-reference/next.config.js/custom-page-extensions#including-non-page-files-in-the-pages-directory
  pageExtensions: ['page.tsx', 'page.ts', 'page.jsx', 'page.js'],
};
