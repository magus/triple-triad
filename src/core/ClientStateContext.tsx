import * as React from 'react';

export type ClientState = {
  scale: number;
};

// hook tuple because we return React.useState
type ContextValue = [ClientState, React.Dispatch<React.SetStateAction<ClientState>>];

const Context = React.createContext<ContextValue>(null);

type Props = {
  children: React.ReactNode;
};

export function ClientStateProvider(props: Props) {
  const state = React.useState<ClientState>({ scale: 1 });

  return <Context.Provider value={state}>{props.children}</Context.Provider>;
}

export function useClientState() {
  const value = React.useContext(Context);

  if (!value) {
    throw new Error('Must wrap tree with <ClientStateProvider>');
  }

  return value;
}
