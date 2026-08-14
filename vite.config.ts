import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import tailwindcss from "@tailwindcss/vite";
import { resolve } from "path";

const host = process.env.TAURI_DEV_HOST;

/**
 * `KIT_UI_FIXTURES=1 npm run dev` swaps the Tauri IPC for a fixture backend so
 * the interface can be designed and inspected without the Rust side running.
 * Off by default, so a normal build never sees it.
 */
const useFixtures = process.env.KIT_UI_FIXTURES === "1";

export default defineConfig(async () => ({
  plugins: [vue(), tailwindcss()],
  resolve: {
    alias: {
      "@": resolve(__dirname, "./src"),
      ...(useFixtures
        ? {
            "@tauri-apps/api/core": resolve(__dirname, "./src/dev/fixtureBackend.ts"),
            "@tauri-apps/api/event": resolve(__dirname, "./src/dev/fixtureEvents.ts"),
          }
        : {}),
    },
  },
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      ignored: ["**/src-tauri/**"],
    },
  },
}));
