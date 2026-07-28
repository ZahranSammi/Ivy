import { fileURLToPath, URL } from "node:url";
import path from "node:path";
import fs from "node:fs";

import { defineConfig, type Plugin } from "vite";
import vue from "@vitejs/plugin-vue";
import tailwindcss from "@tailwindcss/vite";

function swaggerServePlugin(): Plugin {
  return {
    name: "vite-plugin-swagger-serve",
    configureServer(server) {
      server.middlewares.use((req, res, next) => {
        const url = req.url?.split("?")[0];
        if (url === "/docs/openapi.yaml" || url === "/swagger/openapi.yaml") {
          const yamlPath = path.resolve(__dirname, "../../docs/api/openapi.yaml");
          if (fs.existsSync(yamlPath)) {
            res.setHeader("Content-Type", "text/yaml; charset=utf-8");
            res.end(fs.readFileSync(yamlPath, "utf-8"));
            return;
          }
        }

        if (url === "/docs" || url === "/swagger" || url === "/docs/" || url === "/swagger/") {
          res.setHeader("Content-Type", "text/html; charset=utf-8");
          res.end(`<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Ivy API — Swagger UI</title>
  <link rel="stylesheet" href="https://unpkg.com/swagger-ui-dist@5/swagger-ui.css" />
  <style>
    html { box-sizing: border-box; overflow: -moz-scrollbars-vertical; overflow-y: scroll; }
    *, *:before, *:after { box-sizing: inherit; }
    body { margin: 0; background: #fafafa; }
  </style>
</head>
<body>
  <div id="swagger-ui"></div>
  <script src="https://unpkg.com/swagger-ui-dist@5/swagger-ui-bundle.js" charset="UTF-8"></script>
  <script src="https://unpkg.com/swagger-ui-dist@5/swagger-ui-standalone-preset.js" charset="UTF-8"></script>
  <script>
    window.onload = function() {
      window.ui = SwaggerUIBundle({
        url: "/docs/openapi.yaml",
        dom_id: '#swagger-ui',
        deepLinking: true,
        presets: [
          SwaggerUIBundle.presets.apis,
          SwaggerUIStandalonePreset
        ],
        plugins: [
          SwaggerUIBundle.plugins.DownloadUrl
        ],
        layout: "StandaloneLayout"
      });
    };
  </script>
</body>
</html>`);
          return;
        }

        next();
      });
    },
  };
}

export default defineConfig({
  plugins: [vue(), tailwindcss(), swaggerServePlugin()],
  resolve: {
    alias: {
      "@": fileURLToPath(new URL("./src", import.meta.url)),
    },
  },
});

