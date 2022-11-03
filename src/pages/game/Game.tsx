import * as React from 'react';
import * as TauriEvents from '@tauri-apps/api/event';
import { DndContext } from '@dnd-kit/core';

import { PlayerHand } from 'src/components/PlayerHand';
import { Board } from 'src/components/Board';
import { isTauriApp } from 'src/core/isTauriApp';
import { DisableSSR } from 'src/components/DisableSSR';

export function Game() {
  React.useEffect(() => {
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

  const [player_hand, set_player_hand] = React.useState(['88', '75', '89', '93', '96']);
  const [npc_hand, set_npc_hand] = React.useState(['1', '2', '3', '4', '5', '6', '7', '8', '9', '10']);
  const [board, set_board] = React.useState(new Array(9).fill(null));

  function handleDragEnd(args) {
    console.debug('[DndContext]', 'handleDragEnd', { args });

    const active_data = args.active.data.current;

    if (active_data.owner === 'player') {
      set_player_hand(createUpdateHand(active_data.id));
    } else if (active_data.owner === 'npc') {
      set_npc_hand(createUpdateHand(active_data.id));
    }

    set_board((b) => {
      const next_board = [...b];
      next_board[args.over.id] = { ...active_data };
      return next_board;
    });
  }

  console.debug({ board, player_hand, npc_hand });

  return (
    <DndContext onDragEnd={handleDragEnd}>
      <div className="ml-[50%] inline-block -translate-x-1/2">
        <div className="flex flex-row items-start">
          <DisableSSR>
            <PlayerHand cards={player_hand} player />

            <div className="ml-4" />

            <Board board={board} />

            <div className="ml-4" />

            <PlayerHand cards={npc_hand} />
          </DisableSSR>
        </div>
      </div>
    </DndContext>
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
