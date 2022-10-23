import { cli } from './cli.js';
import * as list from './list.js';
import * as json from './json.js';
import path from 'path';

const REPO_ROOT = cli('git rev-parse --show-toplevel');
const GAME_DIR = path.join(REPO_ROOT, 'data', 'game');

export const game_dir = (...parts) => path.join(GAME_DIR, ...parts);

const TRIBE_LIST = json.read(game_dir('tribes.json'));

export const TRIBE = list.to_map((t) => t.id, TRIBE_LIST);
