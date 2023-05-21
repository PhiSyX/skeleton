import * as path from "node:path";

import { defineConfig } from "vite";

import react from "@vitejs/plugin-react";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  resolve: {
    alias: [
      // Assets
      {
        find: /^project:assets:~/,
        replacement: path.resolve("..", "..", "assets"),
      },
      {
        find: /^app:assets:~/,
        replacement: path.resolve("assets"),
      },

      // root src
      {
        find: /^~/,
        replacement: path.resolve("src"),
      },
    ],
  },
  envDir: path.resolve("..", "..", "env", "vite-app"),
});
