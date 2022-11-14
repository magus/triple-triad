import Image from 'next/image';

import CardSpritesheet from './card-spritesheet.png';
import BackgroundGray from './background-gray.png';
import BackgroundBlue from './background-blue.png';
import BackgroundRed from './background-red.png';

import { Draggable } from 'src/components/Draggable';
import * as AppState from 'src/core/AppStateContext';
import * as ClientState from 'src/core/ClientStateContext';
import { Card as TCard } from 'src/core/AppState';

type Props = TCard & {
  index?: number;
  board?: boolean;
  highlight?: boolean;
  selected?: boolean;
  order?: boolean;
};

export function Card(props: Props) {
  const [state] = AppState.useAppState();
  const game_command = AppState.useGameCommand();
  const all_open = ClientState.useAllOpen();
  const three_open = ClientState.useThreeOpen();
  const swap = ClientState.useSwap();

  const id = props.name;
  const image_id = props.id;
  const card = props.index;
  const highlight = props.highlight;
  const modifier = props.modifier;

  let owner;
  let draggable = false;

  if (props.is_player) {
    owner = 'player';
  } else {
    owner = 'npc';
  }

  // only setup draggable when status is null
  if (!state.status) {
    if (props.is_player) {
      draggable = state.turn_is_player === true;
    } else {
      draggable = state.turn_is_player === false;
    }

    // allow either player to act first for convenience
    if (state.game.turn === 0) {
      draggable = true;
    }
  }

  // ensure cards on board cannot be dragged
  if (props.board) {
    draggable = false;
  }

  // if order specified only allow it as draggable
  if (props.is_player && typeof props.order === 'boolean' && draggable) {
    draggable = props.order;
  }

  let onClick;
  let selected;

  if (state.status === AppState.Status.chaos_select) {
    onClick = async function handleClick() {
      // console.debug('[Card]', props.id, 'handleClick');

      await game_command('chaos_select', { card });
      await game_command('explore');
    };
  } else if (!props.is_player && state.status === AppState.Status.all_open) {
    if (typeof card === 'number') {
      selected = props.is_guaranteed || all_open.selected.has(card);

      onClick = () => {
        if (!props.is_guaranteed) {
          all_open.toggle(card);
        }
      };
    }
  } else if (!props.is_player && state.status === AppState.Status.three_open) {
    if (typeof card === 'number') {
      selected = three_open.selected.has(card);
      onClick = () => three_open.toggle(card);
    }
  } else if (state.status === AppState.Status.swap) {
    if (typeof card === 'number') {
      if (props.is_player) {
        selected = swap.player === card;
        onClick = () => swap.select_player(card);
      } else {
        selected = swap.computer === card;
        onClick = () => swap.select_computer(card);
      }
    }
  }

  const config = { id, image_id, owner, modifier, draggable, highlight, selected, onClick };
  // console.debug(config);

  return <DraggableCard {...config} />;
}

type InternalProps = {
  id: string;
  image_id: number;
  owner: 'player' | 'npc' | 'none';
  modifier?: number;
  draggable?: boolean;
  highlight?: boolean;
  selected?: boolean;
  onClick?(): void;
};

function DraggableCard(props: InternalProps) {
  if (!props.image_id) {
    return <EmptyCard />;
  }

  if (props.draggable) {
    const { id, owner } = props;

    return (
      <Draggable id={props.id} data={{ id, owner }} className="relative z-10">
        <CardInternal {...props} />
        <Card.Highlight show={Boolean(props.highlight)} />
      </Draggable>
    );
  }

  return <CardInternal {...props} />;
}

export function CardInternal(props: InternalProps) {
  const x_offset = -1 * card_style.width * (props.image_id - 1);
  const background = getBackground(props.owner);

  const card_size = Card.useCardSize();
  const [client_state] = ClientState.useClientState();

  const container_classNames = ['relative'];

  if (typeof props.selected === 'boolean') {
    if (!props.selected) {
      container_classNames.push('opacity-30');
    }
  }

  const Container = props.onClick ? 'button' : 'div';

  return (
    <Container className={container_classNames.join(' ')} style={{ ...card_size }} onClick={props.onClick}>
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

      <Modifier {...props} />
    </Container>
  );
}

function Modifier(props: InternalProps) {
  if (!props.modifier) {
    return null;
  }

  let is_positive = props.modifier > 0;
  let sign = is_positive ? '+' : '-';

  let shadow_color = '';

  if (is_positive) {
    shadow_color = 'shadow-blue-600';
  } else {
    shadow_color = 'shadow-red-600';
  }

  return (
    <div
      className={`text-shadow absolute top-0 left-0 z-30 flex h-full w-full items-center justify-center text-5xl font-extrabold ${shadow_color}`}
    >
      {sign}
      {props.modifier}
    </div>
  );
}

type HightlightProps = {
  show: boolean;
};

Card.Highlight = function Highlight(props: HightlightProps) {
  const card_size = Card.useCardSize();

  if (!props.show) {
    return null;
  }

  const boxShadow = '0px 0px 16px 8px rgba(34, 197, 94, 0.9)';

  const width = card_size.width;
  const height = card_size.height;

  return (
    <div className="absolute top-0 left-0 h-full w-full">
      <div className="flex h-full w-full items-center justify-center">
        <div className="border-1 rounded-3xl border-green-500" style={{ boxShadow, width, height }} />
      </div>
    </div>
  );
};

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
  const [client_state] = ClientState.useClientState();

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
