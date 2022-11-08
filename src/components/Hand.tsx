import { Card } from 'src/components/Card';
import { useAppState } from 'src/core/AppStateContext';
import { useExploreResult } from 'src/hooks/useExploreResult';

Hand.Player = function Hand_Player() {
  const [state] = useAppState();
  const explore_result = useExploreResult();

  const highlight = explore_result?.card;
  const cards = state.game.player.cards;

  return <Hand {...{ cards, highlight }} />;
};

Hand.Computer = function Hand_Computer() {
  const [state] = useAppState();

  const cards = state.game.computer.cards;

  return <Hand {...{ cards }} />;
};

type Props = {
  highlight?: number;
  cards: Array<CardProps>;
};

type CardProps = React.ComponentProps<typeof Card>;

export function Hand(props: Props) {
  const children: Array<React.ReactNode> = [];

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

  return <div className="flex flex-col items-center">{children}</div>;
}
