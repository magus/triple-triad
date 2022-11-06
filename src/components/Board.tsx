import { Tile } from 'src/components/Tile';
import { Card } from 'src/components/Card';
import { useAppState } from 'src/core/AppStateContext';

export function Board() {
  const state = useAppState();
  const board = state.game.board;

  return <BoardInternal {...{ board }} />;
}

type Props = {
  board: Array<CardProps>;
};

type CardProps = React.ComponentProps<typeof Card>;

function BoardInternal(props: Props) {
  return (
    <div className="inline-block">
      <div className="flex flex-row">
        <BoardTile id="0" {...props} />
        <BoardTile id="1" {...props} />
        <BoardTile id="2" {...props} />
      </div>
      <div className="flex flex-row">
        <BoardTile id="3" {...props} />
        <BoardTile id="4" {...props} />
        <BoardTile id="5" {...props} />
      </div>
      <div className="flex flex-row">
        <BoardTile id="6" {...props} />
        <BoardTile id="7" {...props} />
        <BoardTile id="8" {...props} />
      </div>
    </div>
  );
}

type BoardTileProps = Props & { id: string };

function BoardTile(props: BoardTileProps) {
  return <Tile id={props.id} card={props.board[props.id]} />;
}
