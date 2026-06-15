import { svelte } from "@sveltejs/vite-plugin-svelte";
import { defineConfig } from "vite";

export default defineConfig({
  plugins: [svelte()],
  root: ".",
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true
  },
  envPrefix: ["VITE_", "TAURI_"],
  build: {
    target: "es2022",
    outDir: "dist",
    emptyOutDir: true
  }
});
