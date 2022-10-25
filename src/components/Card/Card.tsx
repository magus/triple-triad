import Image from 'next/image';

import CardSpritesheet from './card-spritesheet.png';
import BackgroundGray from './background-gray.png';
import BackgroundBlue from './background-blue.png';
import BackgroundRed from './background-red.png';

type Props = {
  id: string;
  player?: boolean;
  npc?: boolean;
  computer?: boolean;
};

export function Card(props: Props) {
  const id_numeric = parseInt(props.id, 10);
  const x_offset = -1 * style.card.width * (id_numeric - 1);

  return (
    <div className="relative" style={{ ...style.card }}>
      <img src={getBackground(props)} />
      <div
        className="absolute top-0 left-0"
        style={{
          ...style.card,
          backgroundImage: `url(${CardSpritesheet.src})`,
          backgroundPositionX: x_offset,
        }}
      />
    </div>
  );
}

function getBackground(props) {
  switch (true) {
    case props.npc:
    case props.computer:
      return BackgroundRed.src;
    case props.player:
      return BackgroundBlue.src;
    default:
      return BackgroundGray.src;
  }
}
const style = {
  card: {
    width: 200,
    height: 252,
  },
};
