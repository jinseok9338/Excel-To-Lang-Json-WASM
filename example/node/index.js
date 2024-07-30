import fs from "fs";
import { parse_excel } from "../../node/rust_project.js";

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
