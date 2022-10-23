import Jimp from 'jimp';

import { game_dir } from '../constants.js';
import * as json from '../json.js';
import * as Card from './card.js';
import { OutputImage } from './OutputImage.js';

const CARD_LIST = json.read(game_dir('cards.json'));

await main();

async function main() {
  const output = new OutputImage({ columns: CARD_LIST.length, rows: 1 });

  for (const card of CARD_LIST) {
    output.write(await Card.create(card));
  }

  output.image.write(game_dir('card-spritesheet.png'));
}
