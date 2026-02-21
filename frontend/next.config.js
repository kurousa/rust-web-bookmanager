/** @type {import('next').NextConfig} */
const nextConfig = {
  output: 'standalone',
  env: {
    API_ROOT_PORT: process.env.API_ROOT_PORT,
    API_ROOT_PROTOCOL: process.env.API_ROOT_PROTOCOL,
    API_ROOT_URL: process.env.API_ROOT_URL,
  },
};

module.exports = nextConfig;
