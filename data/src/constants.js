import { cli } from './cli.js';
import path from 'path';

const REPO_ROOT = cli('git rev-parse --show-toplevel');
const GAME_DIR = path.join(REPO_ROOT, 'data', 'game');

export const game_dir = (...parts) => path.join(GAME_DIR, ...parts);
