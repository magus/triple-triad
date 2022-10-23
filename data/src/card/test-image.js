import Jimp from 'jimp';

import { game_dir } from '../constants.js';
import * as json from '../json.js';
import * as Card from './card.js';

const CARDS = json.read(game_dir('cards.json'));
// console.debug({ CARDS });

await main();

async function main() {
  const params = {
    id: '74',
    background: 'gray',
    stars: 5,
    top: 9,
    right: 10,
    bottom: 9,
    left: 1,
  };

  const output = (function (columns, rows) {
    let x = 0;
    let y = 0;
    const card_width = Card.width + 16;
    const card_height = Card.height + 16;

    const image = new Jimp(card_width * 5, card_height * 5, 'transparent');

    function move_right() {
      const result = [x, y];
      x += card_width;
      return result;
    }

    function move_down() {
      const result = [x, y];
      y += card_height;
      x = 0;
      return result;
    }

    function write(add_image) {
      image.composite(add_image, ...move_right());
    }

    return { write, move_down, image };
  })();

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
