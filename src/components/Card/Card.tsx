import Image from 'next/image';

import { Draggable } from 'src/components/Draggable';

import CardSpritesheet from './card-spritesheet.png';
import BackgroundGray from './background-gray.png';
import BackgroundBlue from './background-blue.png';
import BackgroundRed from './background-red.png';

type Props = {
  id: null | string;
  owner: 'player' | 'npc' | 'none';
  draggable?: boolean;
};

export function Card(props: Props) {
  if (props.id === null) {
    return <EmptyCard />;
  }

  if (props.draggable) {
    const { id, owner } = props;

    return (
      <Draggable id={props.id} data={{ id, owner }} className="z-10">
        <CardInternal {...props} />
      </Draggable>
    );
  }

  return <CardInternal {...props} />;
}

function CardInternal(props: Props) {
  const id_numeric = parseInt(props.id, 10);
  const x_offset = -1 * card_style.width * (id_numeric - 1);

  const background = getBackground(props.owner);

  return (
    <div className="relative" style={{ ...card_style }}>
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

function EmptyCard() {
  return <div className="relative" style={{ ...card_style }} />;
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

export const card_style = {
  width: 200,
  height: 252,
};

const style = {
  spritesheet: {
    backgroundImage: `url(${CardSpritesheet.src})`,
  },
};
