import fs from 'fs';
import * as csv from 'csv-parse/sync';
import * as list from '../list.js';
import { game_dir } from '../constants.js';

export function parse_rules() {
  const RuleList = csv.parse(fs.readFileSync(game_dir('rawexd', 'TripleTriadRule.csv')).toString(), {
    from_line: 5,
    on_record: (record, options) => {
      if (!record.name) {
        return null;
      }

      const { id, name, description } = record;

      const output_record = { id, name, description };

      return output_record;
    },

    columns: ['id', 'name', 'description', '', '', '', '', ''],
  });

  // list.preview(RuleList);
  const RuleMap = list.to_map((r) => r.id, RuleList);

  return {
    list: RuleList,
    map: RuleMap,
  };
}
