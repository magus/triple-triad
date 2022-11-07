import * as React from 'react';
import { invoke } from '@tauri-apps/api/tauri';

import { isTauriApp } from 'src/core/isTauriApp';
import { AppState } from 'src/core/AppState';

// hook tuple because we return React.useState
type ContextValue = [AppState, React.Dispatch<React.SetStateAction<AppState>>];

const Context = React.createContext<ContextValue>(null);

type Props = {
  children: React.ReactNode;
};

export function AppStateProvider(props: Props) {
  const state = React.useState<AppState | null>(null);

  return <Context.Provider value={state}>{props.children}</Context.Provider>;
}

export function useGameCommand() {
  const [, set_state] = useAppState();

  async function game_command(name, args?) {
    if (!isTauriApp()) return console.debug('[game_command]', { name, args });

    const start = performance.now();
    invoke(name, args).then(set_state);
    const duration = performance.now() - start;
    console.debug('[game_command]', { duration, name, args });
  }

  return game_command;
}

export function useAppState() {
  const value = React.useContext(Context);

  if (!value) {
    throw new Error('Must wrap tree with <AppStateProvider>');
  }

  return value;
}
