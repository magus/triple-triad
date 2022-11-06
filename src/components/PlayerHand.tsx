import { Card } from 'src/components/Card';
import { useAppState } from 'src/core/AppStateContext';

PlayerHand.Player = function PlayerHand_Player() {
  const state = useAppState();

  const cards = state.game.player.cards;

  return <PlayerHand {...{ cards }} />;
};

PlayerHand.Computer = function PlayerHand_Computer() {
  const state = useAppState();

  const cards = state.game.computer.cards;

  return <PlayerHand {...{ cards }} />;
};

type Props = {
  cards: Array<CardProps>;
};

type CardProps = React.ComponentProps<typeof Card>;

export function PlayerHand(props: Props) {
  const children = [];

  let row = [];

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

    row.push(<Card key={key()} {...card} />);

    if (i && i % 3 === 2) {
      finishRow();
    } else {
      row.push(<div key={key()} className="ml-8" />);
    }
  }

  finishRow();

  return <div className="flex flex-col items-center">{children}</div>;
}
