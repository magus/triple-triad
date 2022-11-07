import * as AppState from 'src/core/AppStateContext';

export function MaybeEndOverlay() {
  const [state] = AppState.useAppState();
  const game_command = AppState.useGameCommand();

  if (!state.is_ended) return null;

  return (
    <div className="pointer-events-none absolute top-0 left-0  z-20 h-full w-full">
      <div className="absolute top-0 left-0 flex h-full w-full bg-black opacity-80" />
      <div className="pointer-events-auto absolute top-0  left-0 flex h-full w-full items-center justify-center">
        <button className="bg-neutral-900 p-16" onClick={() => game_command('reset')}>
          <span className="text-4xl text-white">Reset</span>
        </button>
      </div>
    </div>
  );
}
