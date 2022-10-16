import fs from "fs";
import path from "path";
import * as csv from "csv-parse/sync";
import { parse } from "./parse/parse.js";

const OUTPUT_DIR = "dist";
if (fs.existsSync(OUTPUT_DIR)) {
  fs.rmSync(OUTPUT_DIR, { recursive: true });
}
fs.mkdirSync(OUTPUT_DIR);

const data = parse();

write_json_list("rules", data);
write_json_list("cards", data);
write_json_list("npcs", data);

function write_json_list(name, data) {
  const output_path = path.join(OUTPUT_DIR, `${name}.json`);
  const output_data = data[name].list;
  fs.writeFileSync(output_path, pretty(output_data));
}

function pretty(obj) {
  return JSON.stringify(obj, null, 2);
}
