import { Tile } from 'src/components/Tile';

export function Board() {
  return (
    <div className="inline-block">
      <div className="flex flex-row">
        <Tile id={1} />
        <Tile id={2} />
        <Tile id={3} />
      </div>
      <div className="flex flex-row">
        <Tile id={4} />
        <Tile id={5} />
        <Tile id={6} />
      </div>
      <div className="flex flex-row">
        <Tile id={7} />
        <Tile id={8} />
        <Tile id={9} />
      </div>
    </div>
  );
}
