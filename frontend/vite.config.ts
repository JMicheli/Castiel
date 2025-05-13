import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import path from "path";

// https://vite.dev/config/
export default defineConfig({
  plugins: [react()],
  define: {
    "process.env.VITE_APP_VERSION": JSON.stringify(
      process.env.npm_package_version
    ),
  },
  resolve: {
    alias: {
      "@api": path.resolve(__dirname, "src/api"),
      "@components": path.resolve(__dirname, "src/components"),
      "@contexts": path.resolve(__dirname, "src/contexts"),
      "@hooks": path.resolve(__dirname, "src/hooks"),
      "@providers": path.resolve(__dirname, "src/providers"),
    },
  },
});
