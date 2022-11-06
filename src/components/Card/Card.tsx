import Image from 'next/image';

import CardSpritesheet from './card-spritesheet.png';
import BackgroundGray from './background-gray.png';
import BackgroundBlue from './background-blue.png';
import BackgroundRed from './background-red.png';

import { Draggable } from 'src/components/Draggable';
import { useAppState } from 'src/core/AppStateContext';
import { Card as TCard } from 'src/core/AppState';

type Props = TCard;

export function Card(props: Props) {
  const state = useAppState();

  const id = props.name;
  const image_id = props.id;

  let owner;
  let draggable;

  if (props.is_player) {
    owner = 'player';
    draggable = state.turn_is_player === true;
  } else {
    owner = 'npc';
    draggable = state.turn_is_player === false;
  }

  return <DraggableCard {...{ id, image_id, owner, draggable }} />;
}

type InternalProps = {
  id: string;
  image_id: number;
  owner: 'player' | 'npc' | 'none';
  draggable?: boolean;
};

function DraggableCard(props: InternalProps) {
  if (!props.image_id) {
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

function CardInternal(props: InternalProps) {
  const x_offset = -1 * card_style.width * (props.image_id - 1);

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
