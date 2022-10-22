import fs from "fs";
import * as csv from "csv-parse/sync";
import * as list from "../list.js";
import { game_dir } from "../constants.js";

export function parse_cards() {
  const CardNameList = csv.parse(
    fs.readFileSync(game_dir("rawexd", "TripleTriadCard.csv")).toString(),
    {
      from_line: 5,
      on_record: (record, options) => {
        if (!record.name) {
          return null;
        }

        const output_record = {
          id: record.id,
          name: record.name,
        };

        return output_record;
      },
      columns: [
        "id",
        "name",
        "",
        "",
        "",
        "StartsWithVowel",
        "",
        "",
        "",
        "Description",
      ],
    }
  );

  // list.preview(CardNameList);
  const CardNameMap = list.to_map((c) => c.id, CardNameList);

  // https://csv.js.org/parse/options
  const CardList = csv.parse(
    fs
      .readFileSync(game_dir("rawexd", "TripleTriadCardResident.csv"))
      .toString(),
    {
      from_line: 5,
      on_record: (record, options) => {
        if (!record.top || record.top === "0") {
          return null;
        }

        const int_field_list = [
          "top",
          "right",
          "bottom",
          "left",
          "type",
          "sort",
          "order",
        ];

        const id = record.id;
        const card_name = CardNameMap[id];

        if (!card_name) {
          throw new Error(`Missing card name for [${id}]`);
        }

        const output_record = {
          id,
          name: card_name.name,
        };

        for (const field of int_field_list) {
          output_record[field] = parseInt(record[field], 10);
        }

        return output_record;
      },
      columns: [
        "id",
        "idMinusOne",
        "top",
        "bottom",
        "left",
        "right",
        "rarity",
        "type",
        "sell",
        "sort",
        "order",
        "UIPriority",
        "UnknownBool",
        "AcquisitionType",
        "Acquisition",
        "Location",
        "Quest",
      ],
    }
  );

  // list.preview(CardList);
  const CardMap = list.to_map((c) => c.id, CardList);

  return {
    list: CardList,
    map: CardMap,
  };
}
