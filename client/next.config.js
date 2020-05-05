module.exports = {
  experimental: {
    rewrites() {
      return [
        {
          source: "/api/graphql",
          destination: "http://localhost:8000/api/graphql"
        }
      ];
    }
  }
};
