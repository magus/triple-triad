import { parse_npcs } from './parse_npcs.js';
import { parse_rules } from './parse_rules.js';
import { parse_cards } from './parse_cards.js';

export function parse() {
  const rules = parse_rules();
  const npcs = parse_npcs();
  const cards = parse_cards();

  return { rules, npcs, cards };
}
