import * as React from 'react';
import * as TauriEvents from '@tauri-apps/api/event';
import { DndContext } from '@dnd-kit/core';
import { invoke } from '@tauri-apps/api/tauri';

import { Hand } from 'src/components/Hand';
import { Board } from 'src/components/Board';
import { isTauriApp } from 'src/core/isTauriApp';
import { AppState } from 'src/core/AppState';
import * as MockAppState from 'src/mocks/AppState';
import { AppStateProvider } from 'src/core/AppStateContext';

export function GameInternal() {
  const [state, set_state] = React.useState<AppState>(null);
  console.debug({ state });
  // console.debug(JSON.stringify(state));

  async function game_command(name, args?) {
    console.debug('[game_command]', { name, args });
    if (!isTauriApp()) return;
    invoke(name, args).then(set_state);
  }

  // sync state with rust app
  React.useEffect(function on_mount() {
    if (isTauriApp()) {
      game_command('state');
    } else {
      // fallback to mock
      set_state(MockAppState.IdleImperial);
    }
  }, []);

  React.useEffect(function listen_select_event() {
    if (!isTauriApp()) return;

    const promise_unlisten = TauriEvents.listen('select', (event) => {
      console.debug('select', { event });
    });

    return () => {
      promise_unlisten.then((unlisten) => {
        unlisten();
      });
    };
  }, []);

  function handleDragEnd(args) {
    // console.debug('[DndContext]', 'handleDragEnd', { args });

    if (args.over) {
      const square = +args.over.id;
      const active_data = args.active.data.current;
      const card_name = args.active.data.current.id;
      const [card_id] = card_name.match(/\d+/);
      const card = +card_id;

      // console.debug({ active_data, card, square });

      // optimistic update local state before command returns
      set_state((_state) => {
        const next_state = clone(_state);

        let hand;

        if (next_state.turn_is_player) {
          hand = next_state.game.player.cards;
        } else {
          hand = next_state.game.computer.cards;
        }

        const hand_card = hand[card];
        const board_square = next_state.game.board[square];

        if (board_square.is_empty && !hand_card.is_empty) {
          // valid move
          console.debug('✅ valid move', { hand_card, board_square });
          hand[card] = null;
          next_state.game.board[square] = hand_card;
        } else {
          console.debug('❌ invalid move');
        }

        return next_state;
      });

      setTimeout(() => game_command('execute_turn', { card, square }), 200);
    }
  }

  if (!state) {
    return <div>Loading...</div>;
  }

  const handle_set_deck = () => game_command('set_deck');
  const handle_set_npc = () => game_command('set_npc', { search: 'idle' });

  return (
    <AppStateProvider state={state}>
      <DndContext onDragEnd={handleDragEnd}>
        <div className="ml-[50%] inline-block -translate-x-1/2">
          <div className="flex w-full flex-row justify-center">
            <button onClick={handle_set_deck}>set_deck</button>
            <div className="w-2" />
            <button onClick={handle_set_npc}>set_npc</button>
          </div>

          <div className="flex flex-row items-start">
            <Hand.Player />

            <div className="ml-4" />

            <Board />

            <div className="ml-4" />

            <Hand.Computer />
          </div>
        </div>
      </DndContext>
    </AppStateProvider>
  );
}

function createUpdateHand(id) {
  return function updateHand(current_hand) {
    const next_hand = [...current_hand];
    next_hand[next_hand.indexOf(id)] = null;
    return next_hand;
  };
}

// import Link from 'next/link';
// import { invoke } from '@tauri-apps/api/tauri';

// const [greeting, set_greeting] = React.useState('');
// const [name, set_name] = React.useState('');

// async function greet() {
//   // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
//   set_greeting(await invoke('greet', { name }));
// }

/*
<Link href="/search-npc">Search NPCs</Link>

<input className="greet-input" onChange={(e) => set_name(e.currentTarget.value)} placeholder="Enter a name..." />
<button type="button" onClick={() => greet()}>
  Greet
</button>
<p>{greeting}</p>
*/

function clone(obj) {
  return JSON.parse(JSON.stringify(obj));
}
