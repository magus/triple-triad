import * as React from 'react';
import * as TauriEvents from '@tauri-apps/api/event';
import { DndContext } from '@dnd-kit/core';
import { invoke } from '@tauri-apps/api/tauri';

import { PlayerHand } from 'src/components/PlayerHand';
import { Board } from 'src/components/Board';
import { isTauriApp } from 'src/core/isTauriApp';
import { AppState } from 'src/core/AppState';
import * as MockAppState from 'src/mocks/AppState';
import { AppStateProvider } from 'src/core/AppStateContext';

export function GameInternal() {
  // const [state, set_state] = React.useState<AppState>(null);
  const [state, set_state] = React.useState<AppState>(MockAppState.IdleImperial);

  React.useEffect(function on_mount() {
    if (!isTauriApp()) return;

    async function run() {
      const next_state: AppState = await invoke('set_npc', { search: 'idle' });
      set_state(next_state);
    }

    run();
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
    console.debug('[DndContext]', 'handleDragEnd', { args });

    if (args.over) {
      const over_id = args.over.id;
      const active_data = args.active.data.current;

      console.debug({ over_id, active_data });
    }
  }

  console.debug({ state });
  // console.debug(JSON.stringify(state));

  if (!state) {
    return <div>Loading...</div>;
  }

  return (
    <AppStateProvider state={state}>
      <DndContext onDragEnd={handleDragEnd}>
        <div className="ml-[50%] inline-block -translate-x-1/2">
          <div className="flex flex-row items-start">
            <PlayerHand.Player />

            <div className="ml-4" />

            <Board />

            <div className="ml-4" />

            <PlayerHand.Computer />
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
