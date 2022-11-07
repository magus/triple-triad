import Image from 'next/image';

import { Droppable } from 'src/components/Droppable';
import { Card, card_style } from 'src/components/Card';
import { useClientState } from 'src/core/ClientStateContext';

import BoardSquare from './board-square.png';

type Props = {
  id: string;
  card?: React.ComponentProps<typeof Card>;
};

export function Tile(props: Props) {
  if (!props.card.is_empty) {
    return (
      <TileContainer {...props}>
        <div className="absolute top-0 left-0 z-10 ml-[50%] mt-[50%] -translate-x-1/2 -translate-y-1/2">
          <Card {...props.card} board />
        </div>
      </TileContainer>
    );
  }

  return (
    <Droppable id={props.id} OverElement={OverElement}>
      <TileContainer {...props} />
    </Droppable>
  );
}

type TileContainerProps = Props & {
  children?: React.ReactNode;
};

function TileContainer(props: TileContainerProps) {
  const dimensions = useDimensions();

  return (
    <div className="relative" style={dimensions}>
      <Image className="rotate-0" src={BoardSquare.src} layout="fixed" alt="tile" priority {...dimensions} />
      <MaybeDark {...props} />

      {props.children}
    </div>
  );
}

function MaybeDark(props: Props) {
  const is_even = +props.id % 2 === 0;

  if (is_even) {
    return null;
  }

  return <div className="absolute top-0 left-0 h-full w-full bg-stone-700 opacity-20" />;
}

function OverElement() {
  const dimensions = useDimensions();
  const card_size = Card.useCardSize();
  const boxShadow = '0px 0px 64px 8px rgba(253, 224, 71, 0.9)';

  return (
    <div className="absolute top-0 left-0 h-full w-full" style={{ ...dimensions }}>
      <div className="flex h-full w-full items-center justify-center">
        <div className="border-2 border-yellow-300" style={{ boxShadow, ...card_size }} />
      </div>
    </div>
  );
}

function useDimensions() {
  const [client_state] = useClientState();

  return {
    width: BoardSquare.width * client_state.scale,
    height: BoardSquare.height * client_state.scale,
  };
}
