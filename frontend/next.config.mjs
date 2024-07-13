/** @type {import('next').NextConfig} */
const nextConfig = {
  redirects: async () => {
    return [
      {
        source: "/@:username",
        destination: "/user/:username",
        permanent: true,
      },
    ];
  },
  experimental: {
    serverActions: {
      allowedOrigins: [
        "literate-robot-wpvrpgr6x4pfgqg-3000.app.github.dev/",
        "localhost:3000"
      ]
    }
  }
};

export default nextConfig;
