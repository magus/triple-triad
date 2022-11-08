import * as React from 'react';
import { invoke } from '@tauri-apps/api/tauri';

import { isTauriApp } from 'src/core/isTauriApp';
import { AppState, Status as AppStateStatus } from 'src/core/AppState';

// hook tuple because we return React.useState

export { AppStateStatus as Status };

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

export function useGameCommand() {
  const [, set_state] = useMaybeAppState();

  async function game_command(name, args?) {
    if (!isTauriApp()) return console.debug('[game_command]', { name, args });

    const start = performance.now();
    await invoke(name, args).then(set_state);
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
