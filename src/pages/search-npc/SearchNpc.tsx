import * as React from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import * as TauriEvents from '@tauri-apps/api/event';

export function SearchNpc() {
  const [npc, set_npc] = React.useState('');

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    set_npc(await invoke('greet', { name: npc }));
  }

  return (
    <div className="container">
      <h1>npc</h1>

      <input className="greet-input" onChange={(e) => set_npc(e.currentTarget.value)} placeholder="Enter a name..." />
      <button type="button" onClick={greet}>
        Greet
      </button>

      <p>{npc}</p>
    </div>
  );
}
