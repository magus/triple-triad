// Adapted from the script below
// https://github.com/mattantonelli/ffxiv-triple-triad/blob/master/lib/tasks/card_images.rake

import Jimp from 'jimp';

import { game_dir } from './constants.js';
import * as json from './json.js';

const CARDS = json.read(game_dir('cards.json'));
// console.debug({ CARDS });

const IMAGES = {};
IMAGES.Star = await Jimp.read(game_dir('images', 'star.png'));
IMAGES.Background = {};
IMAGES.Background.gray = await Jimp.read(game_dir('images', 'background-gray.png'));
IMAGES.Background.blue = await Jimp.read(game_dir('images', 'background-blue.png'));
IMAGES.Background.red = await Jimp.read(game_dir('images', 'background-red.png'));

const { scale, width, height } = await (async function () {
  // read in first card to capture measurements
  const image = await create_card_base({ id: '1', background: 'gray' });

  // 208x256    hd card image
  // 200x252    actual card dimensions excluding padding
  // we can autocrop to actual card dimensions
  image.autocrop();

  const width = image.getWidth();
  const height = image.getHeight();

  // constants below are calculated by using reference photos from game
  // see https://www.figma.com/file/O39AUh8Rbog2MQRykgrorO/FFXIV-Triple-Triad?node-id=0%3A1
  // 90 is the width of the reference card in figma
  const scalar = width / 90;

  function scale(value) {
    return Math.round(value * scalar);
  }

  return { scale, width, height };
})();

IMAGES.Number = {};
for (let i = 0; i < 10; i++) {
  const num = String(i + 1);

  const image = await Jimp.read(number_image_path(num));

  // the numbers are 15x15 in figma reference card, so we scale them
  image.resize(scale(15), scale(15));

  IMAGES.Number[num] = image;
}

IMAGES.Tribe = {};
IMAGES.Tribe.garlean = await Jimp.read(game_dir('images', 'garlean.png'));
IMAGES.Tribe.primal = await Jimp.read(game_dir('images', 'primal.png'));
IMAGES.Tribe.scion = await Jimp.read(game_dir('images', 'scion.png'));
IMAGES.Tribe.beastman = await Jimp.read(game_dir('images', 'beastman.png'));

const POS = {};
POS.StarList = [
  [scale(13), scale(4)],
  [scale(5), scale(9)],
  [scale(21), scale(9)],
  [scale(9), scale(17)],
  [scale(18), scale(17)],
];

POS.NumberList = [
  { side: 'top', pos: [scale(37), scale(80)] },
  { side: 'right', pos: [scale(48), scale(85)] },
  { side: 'bottom', pos: [scale(37), scale(91)] },
  { side: 'left', pos: [scale(26), scale(85)] },
];

POS.Tribe = [scale(70), scale(1)];

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
    const card_width = width + 16;
    const card_height = height + 16;

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

  for (const background of Object.keys(IMAGES.Background)) {
    output.write(await create_card({ ...params, background }));
  }

  output.move_down();

  for (let i = 0; i < 5; i++) {
    const stars = i + 1;

    output.write(await create_card({ ...params, stars }));
  }

  output.move_down();

  for (const tribe of Object.keys(IMAGES.Tribe)) {
    output.write(await create_card({ ...params, tribe }));
  }

  output.move_down();

  for (const number of Object.keys(IMAGES.Number)) {
    const left = number;
    output.write(await create_card({ ...params, left }));
    if (number === '5') {
      output.move_down();
    }
  }

  output.image.write('output.png');
}

async function create_card(params) {
  const image = await create_card_base(params);

  const star_count = params.stars;

  // draw stars in reverse order so 4 and 5 are under 2 and 3
  for (let i = 0; i < star_count; i++) {
    const [x, y] = POS.StarList[star_count - i - 1];
    image.composite(IMAGES.Star, x, y);
  }

  for (const num_pos of POS.NumberList) {
    const [x, y] = num_pos.pos;
    const num_image = IMAGES.Number[params[num_pos.side]];

    image.composite(num_image, x, y);
  }

  if (params.tribe) {
    const [x, y] = POS.Tribe;
    image.composite(IMAGES.Tribe[params.tribe], x, y);
  }

  return image;
}

async function create_card_base(params) {
  // start with clone of background and composite on top of it
  const image = IMAGES.Background[params.background].clone();

  const card_id = params.id;
  const card_image = await Jimp.read(card_image_path(card_id));
  image.composite(card_image, 0, 0);

  return image;
}

function card_image_path(card_id) {
  return game_image_path(82100, card_id);
}

function number_image_path(number) {
  return game_image_path(76539, number);
}

function game_image_path(id_start, id) {
  // folders are in groups of 1000, so calculate folder from id_start
  const offset = id_start % 1000;
  const folder_start = String(id_start - offset);
  const zero_pad = Array(6 - folder_start.length).fill('0');
  const folder = [...zero_pad, folder_start].join('');

  const numeric_id = parseInt(id, 10);
  const icon_id = id_start + numeric_id;

  return game_dir('ui', 'icon', folder, `0${icon_id}_hr1.png`);
}
