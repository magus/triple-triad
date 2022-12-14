import Image from 'next/image';

import { Droppable } from 'src/components/Droppable';
import { Card, card_style } from 'src/components/Card';
import { useClientState } from 'src/core/ClientStateContext';

import BoardSquare from './board-square.png';

type Props = {
  id: string;
  highlight?: boolean;
  card?: React.ComponentProps<typeof Card>;
};

export function Tile(props: Props) {
  const card = props.card;

  if (card && !card.is_empty) {
    return (
      <TileContainer {...props}>
        <div className="absolute top-0 left-0 z-10 ml-[50%] mt-[50%] -translate-x-1/2 -translate-y-1/2">
          <Card {...card} board />
        </div>
      </TileContainer>
    );
  }

  return (
    <Droppable id={props.id} OverElement={OverElement}>
      <TileContainer {...props} />
      <Card.Highlight show={Boolean(props.highlight)} />
    </Droppable>
  );
}

type TileContainerProps = Props & {
  children?: React.ReactNode;
};

function TileContainer(props: TileContainerProps) {
  const dimensions = Tile.useDimensions();

  return (
    <div className="relative" style={dimensions}>
      <Image className="rotate-0" src={BoardSquare.src} layout="fixed" alt="tile" priority {...dimensions} />
      <ColorOverlay {...props} />

      {props.children}
    </div>
  );
}

function ColorOverlay(props: Props) {
  const classNames = 'absolute top-0 left-0 h-full w-full';

  let even_overlay;
  const is_even = +props.id % 2 === 0;
  if (is_even) {
    even_overlay = <div className={`${classNames} bg-stone-700 opacity-20`} />;
  }

  const card = props.card;
  let owner_overlay;
  if (card && !card.is_empty) {
    const bg = card.is_player ? 'bg-blue-500' : 'bg-red-500';
    owner_overlay = <div className={`${classNames} ${bg} opacity-40`} />;
  }

  return (
    <div className={`${classNames}`}>
      {even_overlay}
      {owner_overlay}
    </div>
  );
}

function OverElement() {
  const dimensions = Tile.useDimensions();
  const card_size = Card.useCardSize();
  const boxShadow = '0px 0px 32px 8px rgba(253, 224, 71, 0.9)';

  return (
    <div className="absolute top-0 left-0 h-full w-full" style={{ ...dimensions }}>
      <div className="flex h-full w-full items-center justify-center">
        <div className="rounded-3xl border-2 border-yellow-300" style={{ boxShadow, ...card_size }} />
      </div>
    </div>
  );
}

Tile.useDimensions = function useDimensions() {
  const [client_state] = useClientState();

  return {
    width: BoardSquare.width * client_state.scale,
    height: BoardSquare.height * client_state.scale,
  };
};
