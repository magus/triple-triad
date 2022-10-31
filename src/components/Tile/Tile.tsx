import Image from 'next/image';

import BoardSquare from './board-square.png';

type Props = {
  id: number;
};

export function Tile(props: Props) {
  const { width, height } = BoardSquare;
  const dimensions = { width, height };

  return (
    <div className="relative" style={dimensions}>
      <Image className="rotate-0" src={BoardSquare.src} layout="fixed" alt="tile" priority {...dimensions} />
      <MaybeDark {...props} />
    </div>
  );
}

function MaybeDark(props: Props) {
  const is_even = props.id % 2 === 0;

  if (is_even) {
    return null;
  }

  return <div className="absolute top-0 left-0 h-full w-full bg-stone-700 opacity-20" />;
}
