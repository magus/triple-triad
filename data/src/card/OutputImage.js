import Jimp from 'jimp';

import * as Card from './card.js';

const DEFAULT_OPTIONS = Object.freeze({
  padding: [0, 0],
  columns: 1,
  rows: 1,
});

export function OutputImage(input_options = {}) {
  const options = Object.assign({}, DEFAULT_OPTIONS, input_options);

  const [pad_x, pad_y] = options.padding;

  let x = 0;
  let y = 0;
  const card_width = Card.width + pad_x;
  const card_height = Card.height + pad_y;

  const image = new Jimp(card_width * options.columns, card_height * options.rows, 'transparent');

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
}
