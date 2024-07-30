compile and run

install rust
[install rust](https://www.rust-lang.org/tools/install)

install wasm-pack
[wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

compile to wasm

```bash
wasm-pack build --target nodejs
```

1. node 환경에서 사용 하기
   위와 같이 nodejs 를 타켓으로 빌드 후에 (...).js 파일에서 export 가 된 parse_excel 함수를 사용하면 됩니다.

```js
import { parse_excel } from "./node/rust_project.js";
import fs from "fs";

async function main() {
  try {
    console.time("parse_excel");
    const fileContent = fs.readFileSync("test-lang.xlsx");

    // Convert the file content to a Uint8Array
    const fileContentUint8Array = new Uint8Array(fileContent);
    const result = await parse_excel(fileContentUint8Array);
    const json_data = JSON.parse(result);
    //save to file

    fs.writeFileSync("test_ko.json", JSON.stringify(json_data.ko));
    fs.writeFileSync("test_en.json", JSON.stringify(json_data.en));
    console.timeEnd("parse_excel");
  } catch (error) {
    console.error("Error parsing Excel:", error);
  }
}

main();
```

2. vite 환경에서 사용 하기

어려운점 vite 는 이제 type: module 만을 지원하기 때문에 node 로 빌드된 js 를 그대로 사용할 수 없습니다. (node 로 빌드된 js 는 esm 형식이 아니기 때문입니다.)
따라서 웹 타켓으로 빌드를 하고 wasm 파일을 따로 로드 해 주어야 합니다.

```bash
wasm-pack build --target web
```

아래와 같이 vite 플러그인을 사용하면 됩니다.

```js
// vite.config.ts
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
```

```js
// scripts/languageUpdate.ts
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
```

```js
// scripts/languageGenerate.ts
import fs from "fs";

import * as wasm from "../../../web/rust_project";
import path from "path";

export async function generate() {
  try {
    console.time("Parse");
    const fileContent = fs.readFileSync("test-lang.xlsx");
    // // Convert the file content to a Uint8Array
    const fileContentUint8Array = new Uint8Array(fileContent);

    const pathNew = path.resolve(
      __dirname,
      "../../../web/rust_project_bg.wasm"
    );
    // 따로 wasm 파일을 로드해야 합니다.
    const webAssembly = await fs.promises.readFile(pathNew);
    await wasm.default(webAssembly);
    const result: string = await wasm.parse_excel(fileContentUint8Array);
    const json_data = JSON.parse(result);
    fs.writeFileSync("test_ko.json", JSON.stringify(json_data.ko));
    fs.writeFileSync("test_en.json", JSON.stringify(json_data.en));
    console.timeEnd("Parse");
  } catch (error) {
    console.error("Error parsing Excel:", error);
  }
}
```

wasm 으로 되어 있는 플러그인이 기존 플러그인 대비 4배 ~ 5배 정도 빠르게 실행됩니다.
기존 node 플러그인 -> 평균 250ms (엑셀 파일 5000줄)
wasm 플러그인 -> 평균 40ms (엑셀 파일 5000줄)
