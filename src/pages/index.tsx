import * as React from 'react';
import Link from 'next/link';
import { invoke } from '@tauri-apps/api/tauri';
import * as TauriEvents from '@tauri-apps/api/event';

export default function App() {
  const [greeting, set_greeting] = React.useState('');
  const [name, set_name] = React.useState('');

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

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    set_greeting(await invoke('greet', { name }));
  }

  return (
    <div data-tauri-drag-region className="container">
      <h1>Greeting</h1>

      <Link href="/SearchNpc">Search NPCs</Link>

      <input className="greet-input" onChange={(e) => set_name(e.currentTarget.value)} placeholder="Enter a name..." />
      <button type="button" onClick={() => greet()}>
        Greet
      </button>

      <p>{greeting}</p>
    </div>
  );
}
