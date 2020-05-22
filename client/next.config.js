const withBundleAnalyzer = require("@next/bundle-analyzer")({
  enabled: process.env.ANALYZE !== undefined,
});

module.exports = withBundleAnalyzer({
  experimental: {
    modern: true,
  },
  poweredByHeader: false,
});
