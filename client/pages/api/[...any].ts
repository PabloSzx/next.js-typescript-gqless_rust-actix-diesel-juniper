import { createProxyMiddleware } from "http-proxy-middleware";

import type { PageConfig } from "next";

export default createProxyMiddleware({
  target: "http://localhost:8000",
  changeOrigin: true,
  ws: true,
});

export const config: PageConfig = {
  api: {
    bodyParser: false,
    externalResolver: true,
  },
};
