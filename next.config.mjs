/** @type {import('next').NextConfig} */
const nextConfig = {
  // The board is one self-contained page for now; serve it at the site root.
  async rewrites() {
    return {
      beforeFiles: [{ source: "/", destination: "/board.html" }],
    };
  },
};

export default nextConfig;
