import * as React from 'react';
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

export function useAppState() {
  const value = React.useContext(Context);

  if (!value) {
    throw new Error('Must wrap tree with <AppStateProvider>');
  }

  return value;
}
