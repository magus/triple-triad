import { Card } from 'src/components/Card';

type Props = {
  cards: Array<string>;
  player?: boolean;
};

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
    const cardId = props.cards[i];

    row.push(<Card key={key()} id={cardId} owner={props.player ? 'player' : 'npc'} />);

    if (i && i % 3 === 2) {
      finishRow();
    } else {
      row.push(<div key={key()} className="ml-8" />);
    }
  }

  finishRow();

  return <div className="flex flex-col items-center">{children}</div>;
}
