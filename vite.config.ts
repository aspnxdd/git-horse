/// <reference types="vitest" />
import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import AutoImport from "unplugin-auto-import/vite";
import { resolve } from "path";
import tsconfigPaths from "vite-tsconfig-paths";

// https://vitejs.dev/config/
export default defineConfig({
  optimizeDeps: {
    exclude: ["oh-vue-icons/icons"],
  },
  clearScreen: false,
  plugins: [
    vue(),
    tsconfigPaths({
      extensions: [".vue", ".js", ".ts"],
    }),
    AutoImport({
      imports: ["vue"],
      dts: "./src/auto-imports.d.ts",
      eslintrc: {
        enabled: true,
        filepath: resolve(__dirname, ".eslintrc-auto-import.json"),
      },
    }),
  ],
  build: {
    outDir: "./dist",
    emptyOutDir: true,
  },
  test: {
    include: ["tests/unit/**/*.{test,spec}.{js,mjs,cjs,ts,mts,cts,jsx,tsx}"],
  },
});
