import * as React from 'react';

export function SearchNpc() {
  const [npc, set_npc] = React.useState('');

  return (
    <div className="container">
      <h1>npc</h1>

      <input className="greet-input" onChange={(e) => set_npc(e.currentTarget.value)} placeholder="Enter a name..." />

      <p>{npc}</p>
    </div>
  );
}
