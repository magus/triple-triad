import * as React from 'react';
import { DndContext } from '@dnd-kit/core';

import { Hand } from 'src/components/Hand';
import { Board } from 'src/components/Board';
import { Button } from 'src/components/Button';
import { isTauriApp } from 'src/core/isTauriApp';
import * as MockAppState from 'src/mocks/AppState';
import * as AppState from 'src/core/AppStateContext';
import { useClientState } from 'src/core/ClientStateContext';
import { MaybeEndOverlay } from 'src/components/MaybeEndOverlay';

export function GameInternal() {
  console.debug('[GameInternal]', 'render');

  return (
    <React.Fragment>
      <Behaviors />
      <MaybeGame />
    </React.Fragment>
  );
}

function MaybeGame() {
  const [state, set_state] = AppState.useMaybeAppState();
  console.debug({ state });
  // console.debug(JSON.stringify(state));

  if (!state) {
    return (
      <div className="flex w-full flex-row justify-center">
        <div className="text-6xl font-bold">Loading...</div>
      </div>
    );
  }

  return (
    <DragZone>
      <div className="ml-[50%] inline-block -translate-x-1/2" id="game-container">
        <Status />
        <Actions />

        <div className="h-4" />

        <GameBoard />

        <MaybeEndOverlay />
      </div>
    </DragZone>
  );
}

function Behaviors() {
  const [, set_state] = AppState.useMaybeAppState();
  const game_command = AppState.useGameCommand();

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

  return null;
}

type DragZoneProps = {
  children: React.ReactNode;
};

function DragZone(props: DragZoneProps) {
  const game_command = AppState.useGameCommand();

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

  return <DndContext onDragEnd={handleDragEnd}>{props.children}</DndContext>;
}

function Status() {
  const [state] = AppState.useAppState();

  return (
    // border-4 border-white
    <div className="flex h-32 items-center justify-center text-4xl">
      {(function () {
        switch (state.status) {
          case AppState.Status.chaos_select:
            return (
              <span>
                Select the card randomly selected by <span className="font-bold">Chaos</span>
              </span>
            );
          default:
            return null;
        }
      })()}
    </div>
  );
}

function Actions() {
  const [state] = AppState.useAppState();
  const game_command = AppState.useGameCommand();

  return (
    <div className="flex w-full flex-row justify-center">
      <Button onClick={() => game_command('set_deck')}>set_deck</Button>
      <div className="w-2" />
      <Button onClick={() => game_command('set_npc', { search: 'idle' })}>set_npc</Button>
      <div className="w-2" />
      <Button onClick={() => game_command('start')}>start</Button>
      <div className="w-2" />
      <Button
        color="green"
        onClick={async () => {
          if (state.turn_is_player && state.game.rules.chaos && !state.game.chaos_card) {
            // set status to chaos to select chaos card for better search
            console.debug('🎲 select chaos card');
            const status = AppState.Status.chaos_select;
            game_command('status', { status });
          } else {
            game_command('explore');
          }
        }}
      >
        explore
      </Button>
      <div className="w-2" />
      <Button color="red" onClick={() => game_command('reset')}>
        reset
      </Button>
    </div>
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
