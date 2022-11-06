import * as React from 'react';
import { DndContext } from '@dnd-kit/core';
import { invoke } from '@tauri-apps/api/tauri';

import { Hand } from 'src/components/Hand';
import { Board } from 'src/components/Board';
import { isTauriApp } from 'src/core/isTauriApp';
import * as MockAppState from 'src/mocks/AppState';
import { useAppState } from 'src/core/AppStateContext';
import { useClientState } from 'src/core/ClientStateContext';

export function GameInternal() {
  const key = React.useRef(0);
  const [state, set_state] = useAppState();

  key.current += 1;
  console.debug(key.current, { state });
  // console.debug(JSON.stringify(state));

  async function game_command(name, args?) {
    if (!isTauriApp()) return console.debug('[game_command]', { name, args });

    const start = performance.now();
    invoke(name, args).then(set_state);
    const duration = performance.now() - start;
    console.debug('[game_command]', { duration, name, args });
  }

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
    const card_name = args.active.data.current.id;
    const [card_id] = card_name.match(/\d+/);
    const card = +card_id;

    // console.debug({ active_data, card, square });
    // wait for execute to finish before updating
    await game_command('execute_turn', { card, square });
  }

  if (!state) {
    return <div>Loading...</div>;
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
      </div>
    </DndContext>
  );
}

function GameBoard() {
  const [client_state, set_client_state] = useClientState();

  const board_container_ref = React.useRef(null);

  // determine window and screen size?
  React.useEffect(function on_mount() {
    const screen = {
      width: window.screen.width,
      height: window.screen.height,
    };

    const body = {
      width: window.document.body.clientWidth,
      height: window.document.body.clientHeight,
    };

    const board = {
      width: 0,
      height: 0,
    };

    if (board_container_ref.current) {
      board.width = board_container_ref.current.offsetWidth;
      board.height = board_container_ref.current.offsetHeight;
    }

    const scale = body.width / board.width;

    console.debug('[dimensions]', { scale, screen, body, board });
  }, []);

  return (
    <div className="flex flex-row items-start px-16" id="board-container" ref={board_container_ref}>
      <Hand.Player />

      <div className="ml-4" />

      <Board />

      <div className="ml-4" />

      <Hand.Computer />
    </div>
  );
}

// import Link from 'next/link';
// <Link href="/search-npc">Search NPCs</Link>

function clone(obj) {
  return JSON.parse(JSON.stringify(obj));
}
