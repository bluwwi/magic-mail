import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  reactCompiler: true,
  async rewrites() {
    return [
      {
        source: "/api/:path*",
        destination: "http://127.0.0.1:3001/api/:path*",
      },
      {
        source: "/sse/:path*",
        destination: "http://127.0.0.1:3001/sse/:path*",
      },
    ];
  },
};

export default nextConfig;
