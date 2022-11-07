export type AppState = {
  status: string;
  game: Game;
  explore_result: null | ExploreResult;

  turn_is_player: boolean;
  is_ended: boolean;
};

type ExploreResult = {
  total_depth_moves: number;
  actual_moves_evaluated: number;
  is_estimate: boolean;
  results: Array<ExploreResultItem>;
};

type ExploreResultItem = {
  game: Game;
  score: number;
};

type Game = {
  board: Board;
  chaos_card: null;
  computer: Computer;
  evaluation_max: number;
  is_player_first: boolean;
  last_move: null;
  player: Player;
  rules: Rules;
  score: number;
  turn: number;
};

export type Card = {
  id: number;
  is_empty: boolean;
  is_guaranteed: boolean;
  is_player: boolean;
  modifier: number;
  name: string;
  sides: [number, number, number, number];
  tribe: number;
};

type Computer = {
  cards: [Card, Card, Card, Card, Card, Card, Card, Card, Card, Card];
  cards_used: number;
};

type Player = {
  cards: [Card, Card, Card, Card, Card];
};

type Board = [Card, Card, Card, Card, Card, Card, Card, Card, Card];

type Rules = {
  all_open: boolean;
  ascension: boolean;
  chaos: boolean;
  descension: boolean;
  draft: boolean;
  fallen_ace: boolean;
  order: boolean;
  plus: boolean;
  random: boolean;
  reverse: boolean;
  roulette: boolean;
  same: boolean;
  sudden_death: boolean;
  swap: boolean;
  three_open: boolean;
};
