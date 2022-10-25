import Image from 'next/image';

import BoardSquare from './board-square.png';

export function Tile() {
  return <Image className="rotate-0 object-contain" src={BoardSquare.src} alt="tile" />;
}
