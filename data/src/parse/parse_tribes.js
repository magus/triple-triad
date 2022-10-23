import fs from 'fs';
import * as csv from 'csv-parse/sync';
import * as list from '../list.js';
import { game_dir } from '../constants.js';

export function parse_tribes() {
  const data_list = csv.parse(fs.readFileSync(game_dir('rawexd', 'TripleTriadCardType.csv')).toString(), {
    from_line: 5,
    on_record: (record, options) => {
      if (!record.name) {
        return null;
      }

      const { id, name } = record;

      const output_record = { id, name };

      return output_record;
    },

    columns: ['id', 'name'],
  });

  // list.preview(data_list);
  const data_map = list.to_map((r) => r.id, data_list);

  return {
    list: data_list,
    map: data_map,
  };
}
