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

  const background = getBackground(props);

  return (
    <div className="relative" style={{ ...style.card }}>
      <Image src={background.src} alt={background.alt} />

      <div
        className="absolute top-0 left-0 h-full w-full"
        style={{
          ...style.spritesheet,
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
      return { src: BackgroundRed.src, alt: 'red' };
    case props.player:
      return { src: BackgroundBlue.src, alt: 'blue' };
    default:
      return { src: BackgroundGray.src, alt: 'gray' };
  }
}

const style = {
  card: {
    width: 200,
    height: 252,
  },

  spritesheet: {
    backgroundImage: `url(${CardSpritesheet.src})`,
  },
};
