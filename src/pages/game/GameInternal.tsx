import * as React from 'react';
import { DndContext } from '@dnd-kit/core';

import { Hand } from 'src/components/Hand';
import { Board } from 'src/components/Board';
import { isTauriApp } from 'src/core/isTauriApp';
import * as MockAppState from 'src/mocks/AppState';
import * as AppState from 'src/core/AppStateContext';
import { useClientState } from 'src/core/ClientStateContext';
import { MaybeEndOverlay } from 'src/components/MaybeEndOverlay';

export function GameInternal() {
  const key = React.useRef(0);
  const [state, set_state] = AppState.useAppState();
  const game_command = AppState.useGameCommand();

  key.current += 1;
  console.debug(key.current, { state });
  // console.debug(JSON.stringify(state));

  // sync state with rust app
  React.useEffect(function on_mount() {
    if (isTauriApp()) {
      game_command('state');
    } else {
      // fallback to mock
      set_state(MockAppState.IdleImperial);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  async function handleDragEnd(args) {
    // console.debug('[DndContext]', 'handleDragEnd', { args });

    if (!args.over) {
      // not over valid droppable, abort drag
      return await game_command('state');
    }

    const square = +args.over.id;
    const active_data = args.active.data.current;
    const card_name = active_data.id;
    const isPlayer = active_data.owner === 'player';
    const [card_id] = card_name.match(/\d+/);
    const card = +card_id;

    console.debug({ card, square, isPlayer });
    // wait for execute to finish before updating
    await game_command('execute_turn', { card, square, isPlayer });
  }

  if (!state) {
    return (
      <div className="flex w-full flex-row justify-center">
        <div className="text-6xl font-bold">Loading...</div>
      </div>
    );
  }

  return (
    <DndContext onDragEnd={handleDragEnd}>
      <div key={key.current} className="ml-[50%] inline-block -translate-x-1/2" id="game-container">
        <div className="flex w-full flex-row justify-center">
          <button onClick={() => game_command('set_deck')}>set_deck</button>
          <div className="w-2" />
          <button onClick={() => game_command('set_npc', { search: 'idle' })}>set_npc</button>
          <div className="w-2" />
          <button onClick={() => game_command('start')}>start</button>
          <div className="w-2" />
          <button onClick={() => game_command('reset')}>reset</button>
        </div>

        <GameBoard />

        <MaybeEndOverlay />
      </div>
    </DndContext>
  );
}

function GameBoard() {
  const [client_state, set_client_state] = useClientState();
  // console.debug({ client_state });

  // determine window size and scale
  React.useEffect(function on_mount() {
    const body_width = window.document.body.clientWidth;
    // this is the width of the board before scaling
    // if we significantly change ui we should recapture this value
    // hardcoded to avoid measuring loops which will cycle
    const original_width = 2304;

    const scale = body_width / original_width;
    const final_scale = +scale.toFixed(2);

    console.debug('[dimensions]', { scale, final_scale, body_width });

    set_client_state((s) => {
      const next_state = { ...s };
      next_state.scale = final_scale;
      return next_state;
    });
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  // adjust font size to scale rem which is used
  // for spacing in tailwind e.g. `px-16`, `mt-2` etc.
  const style = `
    html {
      font-size: ${Math.round(16 * client_state.scale)}px !important;
    }
  `;

  return (
    <div className="flex flex-row items-start px-16" id="board-container">
      <Hand.Player />

      <div className="ml-4" />

      <Board />

      <div className="ml-4" />

      <Hand.Computer />

      <style>{style}</style>
    </div>
  );
}

// import Link from 'next/link';
// <Link href="/search-npc">Search NPCs</Link>

function clone(obj) {
  return JSON.parse(JSON.stringify(obj));
}
