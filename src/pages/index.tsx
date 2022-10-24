import * as React from "react";
import { invoke } from "@tauri-apps/api/tauri";

function App() {
  const [greeting, set_greeting] = React.useState("");
  const [name, set_name] = React.useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    set_greeting(await invoke("greet", { name }));
  }

  return (
    <div className="container">
      <h1>Greeting</h1>

      <input
        className="greet-input"
        onChange={(e) => set_name(e.currentTarget.value)}
        placeholder="Enter a name..."
      />
      <button type="button" onClick={() => greet()}>
        Greet
      </button>

      <p>{greeting}</p>
    </div>
  );
}

export default App;
