import fs from "fs";
import * as csv from "csv-parse/sync";
import { parse_npcs } from "./parse_npcs.js";
import { parse_rules } from "./parse_rules.js";
import { parse_cards } from "./parse_cards.js";

const rules = parse_rules();
const npcs = parse_npcs();
const cards = parse_cards();
