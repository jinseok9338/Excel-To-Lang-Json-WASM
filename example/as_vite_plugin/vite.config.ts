import { defineConfig } from "vite";
import react from "@vitejs/plugin-react-swc";
import languageUpdate from "./scripts/languageUpdate";

const i18nConfig = {
  excelFile: "language.xlsx",
  support: ["ko", "en"],
  outputPath: "src/lang",
};

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react(), languageUpdate(i18nConfig)],
});
