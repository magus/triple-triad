import Image from 'next/image';

import CardSpritesheet from './card-spritesheet.png';
import BackgroundGray from './background-gray.png';
import BackgroundBlue from './background-blue.png';
import BackgroundRed from './background-red.png';

import { Draggable } from 'src/components/Draggable';
import { useAppState } from 'src/core/AppStateContext';
import { useClientState } from 'src/core/ClientStateContext';
import { Card as TCard } from 'src/core/AppState';

type Props = TCard & {
  board?: boolean;
};

export function Card(props: Props) {
  const [state] = useAppState();

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

  // allow either player to act first for convenience
  if (state.game.turn === 0) {
    draggable = true;
  }

  // ensure cards on board cannot be dragged
  if (props.board) {
    draggable = false;
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

  const card_size = Card.useCardSize();
  const [client_state] = useClientState();

  return (
    <div className="relative" style={{ ...card_size }}>
      <Image {...background} alt={background.alt} layout="fill" priority />

      <div
        className="absolute top-0 left-0"
        style={{
          ...style.spritesheet,
          ...card_style,
          backgroundPositionX: x_offset,
          transform: `scale(${client_state.scale})`,
          transformOrigin: 'top left',
        }}
      />
    </div>
  );
}

function EmptyCard() {
  const card_size = Card.useCardSize();
  return <div className="relative" style={{ ...card_size }} />;
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
  const { src } = background;
  return { src };
}

Card.useCardSize = function useCardSize() {
  const [client_state] = useClientState();

  return {
    width: card_style.width * client_state.scale,
    height: card_style.height * client_state.scale,
  };
};

export const card_style = {
  width: 200,
  height: 252,
};

const style = {
  spritesheet: {
    backgroundImage: `url(${CardSpritesheet.src})`,
  },
};
