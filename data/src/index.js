import * as csv from "csv-parse/sync";

import { parse } from "./parse/parse.js";
import { cli } from "./cli.js";
import * as json from "./json.js";
import { game_dir } from "./constants.js";

// unzip game data so it is available
process.chdir(game_dir());
cli("unzip -oq data.zip");

const data = parse();

write_json_list("rules", data);
write_json_list("cards", data);
write_json_list("npcs", data);

function write_json_list(name, data) {
  const output_path = game_dir(`${name}.json`);
  const output_data = data[name].list;
  json.write(output_path, output_data);
}
