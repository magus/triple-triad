import * as React from 'react';
import { invoke } from '@tauri-apps/api/tauri';

import { isTauriApp } from 'src/core/isTauriApp';
import { AppState } from 'src/core/AppState';
import * as ClientState from 'src/core/ClientStateContext';

export { Status } from 'src/core/AppState';
export type { Npc, CardJson } from 'src/core/AppState';

// hook tuple because we return React.useState
type ContextValue = NullContextValue | ValidContextValue;
type ValidContextValue = [AppState, React.Dispatch<React.SetStateAction<AppState>>];
type NullContextValue = [null, React.Dispatch<React.SetStateAction<AppState>>];

const Context = React.createContext<null | ContextValue>(null);

type Props = {
  children: React.ReactNode;
};

export function AppStateProvider(props: Props) {
  const state = React.useState(null);

  // @ts-ignore this is being annoying, skip for now
  return <Context.Provider value={state}>{props.children}</Context.Provider>;
}

export function useCommand() {
  async function command(name, args?) {
    if (!isTauriApp()) return console.debug('[command]', { name, args });

    const start = performance.now();
    const result = await invoke(name, args);
    const duration = Math.round(performance.now() - start);
    console.debug('[command]', { duration, name, args });

    return result;
  }

  return command;
}

export function useGameCommand() {
  const [, set_state] = useMaybeAppState();
  const client_reset = ClientState.useReset();

  async function game_command(name, args?) {
    if (!isTauriApp()) return console.debug('[game_command]', { name, args });

    const start = performance.now();
    await invoke(name, args).then(set_state);
    client_reset();
    const duration = Math.round(performance.now() - start);
    console.debug('[game_command]', { duration, name, args });
  }

  return game_command;
}

export function useMaybeAppState() {
  const value = React.useContext(Context);

  if (!value) {
    throw new Error('Must wrap tree with <AppStateProvider>');
  }

  return value;
}

export function useAppState() {
  const value = React.useContext(Context);

  if (!value) {
    throw new Error('Must wrap tree with <AppStateProvider>');
  }

  const [state] = value;

  if (!state) {
    throw new Error('AppState cannot be accessed before initialized');
  }

  return value;
}
