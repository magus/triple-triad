import { Tile } from 'src/components/Tile';

export function Board() {
  return (
    <div className="inline-block">
      <div className="flex flex-row">
        <Tile />
        <Tile />
        <Tile />
      </div>
      <div className="flex flex-row">
        <Tile />
        <Tile />
        <Tile />
      </div>
      <div className="flex flex-row">
        <Tile />
        <Tile />
        <Tile />
      </div>
    </div>
  );
}
