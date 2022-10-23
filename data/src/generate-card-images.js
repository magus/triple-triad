// Adapted from the script below
// https://github.com/mattantonelli/ffxiv-triple-triad/blob/master/lib/tasks/card_images.rake

import Jimp from 'jimp';

import { game_dir } from './constants.js';
import * as json from './json.js';

const cards = json.read(game_dir('cards.json'));
// console.debug({ cards });

async function main() {
  // start with background and composite on top of it
  const image = await Jimp.read(game_dir('images', 'background-blue.png'));

  // read in first card to capture measurements
  const card_id = '74';
  const card_image = await Jimp.read(card_image_path(card_id));
  image.composite(card_image, 0, 0);

  // 208x256    hd card image
  // 200x252    actual card dimensions excluding padding
  // we can autocrop to actual card dimensions
  image.autocrop();

  const star_size = 13;
  const card_width = 90;
  const card_height = 114;

  // calculated by using reference photos from game
  // see https://www.figma.com/file/O39AUh8Rbog2MQRykgrorO/FFXIV-Triple-Triad?node-id=0%3A1
  const star_pos_list = [
    [13, 4],
    [5, 9],
    [21, 9],
    [9, 17],
    [18, 17],
  ];

  const num_pos_list = [
    { side: 'top', pos: [37, 80] },
    { side: 'right', pos: [48, 85] },
    { side: 'bottom', pos: [37, 91] },
    { side: 'left', pos: [26, 85] },
  ];

  const scalar = image.getWidth() / card_width;

  const star_image = await Jimp.read(game_dir('images', 'star.png'));

  const star_count = 5;

  // draw stars in reverse order so 4 and 5 are under 2 and 3
  for (let i = 0; i < star_count; i++) {
    const star_pos = star_pos_list[star_count - i - 1];

    const [x, y] = star_pos.map((n) => Math.round(n * scalar));

    image.composite(star_image, x, y);
  }

  const card = {
    top: 9,
    right: 3,
    bottom: 9,
    left: 8,
  };

  const scale_num_image = Math.round(15 * scalar);

  const number_image_map = {};
  for (let i = 0; i < 10; i++) {
    const num = String(i + 1);
    const image = await Jimp.read(number_image_path(num));
    image.resize(scale_num_image, scale_num_image);
    number_image_map[num] = image;
  }

  for (const num_pos of num_pos_list) {
    const [x, y] = num_pos.pos.map((n) => Math.round(n * scalar));
    const num_image = number_image_map[card[num_pos.side]];

    image.composite(num_image, x, y);
  }

  const tribe_image = await Jimp.read(game_dir('images', 'garlean.png'));
  const [x, y] = [70, 1].map((n) => Math.round(n * scalar));
  image.composite(tribe_image, x, y);

  image.write(`${card_id}.png`);
}

main();

// let image = new Jimp(300, 530, "green", (err, image) => {
//   if (err) throw err;
// });

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
