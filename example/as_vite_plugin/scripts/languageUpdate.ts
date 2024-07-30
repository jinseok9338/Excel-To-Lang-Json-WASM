import { Plugin } from "vite";
import { generate } from "./languageGenerate";
import path from "path";

export default function languageUpdate(config: { excelFile: string }): Plugin {
  return {
    name: "language-update",
    enforce: "pre",

    configureServer(server) {
      const listener = (file = "") =>
        file.includes(path.normalize(config.excelFile)) ? generate() : null;
      server.watcher.on("add", listener);
      server.watcher.on("change", listener);
    },
    buildStart(): Promise<void> {
      generate();
      return Promise.resolve();
    },
  };
}
