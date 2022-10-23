import { parse_npcs } from './parse_npcs.js';
import { parse_rules } from './parse_rules.js';
import { parse_cards } from './parse_cards.js';
import { parse_tribes } from './parse_tribes.js';

export function parse() {
  const rules = parse_rules();
  const npcs = parse_npcs();
  const cards = parse_cards();
  const tribes = parse_tribes();

  return { rules, npcs, cards, tribes };
}
