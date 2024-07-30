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
