import * as React from 'react';
import * as TauriEvents from '@tauri-apps/api/event';

import { PlayerHand } from 'src/components/PlayerHand';
import { Board } from 'src/components/Board';

export function Game() {
  React.useEffect(() => {
    const promise_unlisten = TauriEvents.listen('select', (event) => {
      console.debug('select', { event });
    });

    return () => {
      promise_unlisten.then((unlisten) => {
        unlisten();
      });
    };
  }, []);

  return (
    <div className="ml-[50%] inline-block -translate-x-1/2">
      <div className="flex flex-row items-center">
        <PlayerHand />

        <div className="ml-4" />

        <Board />

        <div className="ml-4" />

        <PlayerHand />
      </div>
    </div>
  );
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
