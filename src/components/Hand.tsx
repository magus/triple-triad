import React from 'react';
import { Card } from 'src/components/Card';
import { Tile } from 'src/components/Tile';
import * as AppState from 'src/core/AppStateContext';
import { useExploreResult } from 'src/hooks/useExploreResult';

export function Player() {
  const [state] = AppState.useAppState();
  const explore_result = useExploreResult();

  const highlight = explore_result?.card;
  const cards = state.game.player.cards;

  return <Hand {...{ cards, highlight }} />;
}

export function Computer() {
  const [state] = AppState.useAppState();

  const game_cards = state.game.computer.cards;

  let guaranteed_card_count = 0;

  if (state.npc?.cards) {
    for (const card of state.game.computer.cards) {
      if (card.is_guaranteed) {
        guaranteed_card_count++;
      }
    }
  }

  let card_count;

  if (guaranteed_card_count === 5) {
    card_count = 5;
  } else {
    card_count = state.npc?.cards.length;
  }

  const cards = game_cards.slice(0, card_count);

  return <Hand {...{ cards }} />;
}

type Props = {
  highlight?: number;
  cards: Array<CardProps>;
};

type CardProps = React.ComponentProps<typeof Card>;

function Hand(props: Props) {
  const children: Array<React.ReactNode> = [];
  const tile_dimensions = Tile.useDimensions();
  const height = tile_dimensions.height * 3;

  let row: Array<React.ReactNode> = [];

  const key = () => `${children.length}-${row.length}`;

  function finishRow() {
    if (children.length) {
      children.push(<div key={key()} className="mt-8" />);
    }
    children.push(
      <div key={key()} className="flex flex-row">
        {row}
      </div>
    );
    row = [];
  }

  for (let i = 0; i < props.cards.length; i++) {
    const card = props.cards[i];

    const highlight = props.highlight === i;

    row.push(<Card key={key()} {...card} highlight={highlight} index={i} />);

    if (i && i % 3 === 2) {
      finishRow();
    } else {
      row.push(<div key={key()} className="ml-8" />);
    }
  }

  finishRow();

  return (
    <div className="flex flex-col items-center" style={{ height }}>
      {children}
    </div>
  );
}
