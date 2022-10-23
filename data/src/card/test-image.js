import Jimp from 'jimp';

import { game_dir } from '../constants.js';
import * as Card from './card.js';
import { OutputImage } from './OutputImage.js';

await main();

async function main() {
  const output = new OutputImage({
    columns: 5,
    rows: 5,
    padding: [32, 32],
  });

  const params = {
    id: '74',
    stars: 5,
    top: 9,
    right: 10,
    bottom: 9,
    left: 1,
  };

  for (const background of Object.keys(Card.IMAGES.Background)) {
    output.write(await Card.create({ ...params, background }));
  }

  output.move_down();

  for (let i = 0; i < 5; i++) {
    const stars = i + 1;

    output.write(await Card.create({ ...params, stars }));
  }

  output.move_down();

  for (const tribe of Object.keys(Card.IMAGES.Tribe)) {
    output.write(await Card.create({ ...params, tribe }));
  }

  output.move_down();

  for (const number of Object.keys(Card.IMAGES.Number)) {
    const left = number;
    output.write(await Card.create({ ...params, left }));
    if (number === '5') {
      output.move_down();
    }
  }

  output.image.write(game_dir('test-image.png'));
}
