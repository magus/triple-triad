import * as AppState from 'src/core/AppStateContext';
import { Tile } from 'src/components/Tile';

export function MaybeEndOverlay() {
  const tile_dimensions = Tile.useDimensions();
  const board_height = tile_dimensions.height * 3;

  const [state] = AppState.useAppState();
  const game_command = AppState.useGameCommand();

  if (!state.is_ended) return null;

  return (
    <div className="pointer-events-none absolute top-0 left-0 z-20 h-full w-full">
      <div className="absolute top-0 left-0 flex h-full w-full bg-black opacity-80" />
      <div
        className="pointer-events-auto absolute top-0 left-0 flex w-full items-center justify-center"
        style={{ height: board_height }}
      >
        <button className="rounded-md bg-neutral-900 p-16" onClick={() => game_command('reset')}>
          <span className="text-4xl font-bold text-white">Reset</span>
        </button>
      </div>
    </div>
  );
}
