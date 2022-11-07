import * as React from 'react';

export type ClientState = {
  scale: number;
  explore_result_index: number;
};

// hook tuple because we return React.useState
type ContextValue = NullContextValue | ValidContextValue;
type ValidContextValue = [ClientState, React.Dispatch<React.SetStateAction<ClientState>>];
type NullContextValue = [null, React.Dispatch<React.SetStateAction<ClientState>>];

const Context = React.createContext<null | ContextValue>(null);

type Props = {
  children: React.ReactNode;
};

export function ClientStateProvider(props: Props) {
  const state = React.useState<ClientState>({
    scale: 1,
    explore_result_index: 0,
  });

  return <Context.Provider value={state}>{props.children}</Context.Provider>;
}

export function useClientState() {
  const value = React.useContext(Context);

  if (!value) {
    throw new Error('Must wrap tree with <ClientStateProvider>');
  }

  const [state] = value;

  if (!state) {
    throw new Error('ClientState cannot be accessed before initialized');
  }

  return value;
}
