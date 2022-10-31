import Image from 'next/image';

import CardSpritesheet from './card-spritesheet.png';
import BackgroundGray from './background-gray.png';
import BackgroundBlue from './background-blue.png';
import BackgroundRed from './background-red.png';

type Props = {
  id: string;
  owner: 'player' | 'npc' | 'none';
};

export function Card(props: Props) {
  const id_numeric = parseInt(props.id, 10);
  const x_offset = -1 * style.card.width * (id_numeric - 1);

  const background = getBackground(props.owner);

  function handleClick() {
    console.debug('[Card]', props.id);
  }

  return (
    <div className="relative" style={{ ...style.card }} onClick={handleClick}>
      <Image {...background} alt={background.alt} priority />

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

function getBackground(owner) {
  switch (owner) {
    case 'npc':
      return { ...getBackgroundProps(BackgroundRed), alt: 'red' };
    case 'player':
      return { ...getBackgroundProps(BackgroundBlue), alt: 'blue' };
    case 'none':
    default:
      return { ...getBackgroundProps(BackgroundGray), alt: 'gray' };
  }
}

function getBackgroundProps(background) {
  const { width, height, src } = background;
  return { width, height, src };
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
