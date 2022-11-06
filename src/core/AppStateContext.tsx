import * as React from 'react';
import { AppState } from 'src/core/AppState';

const Context = React.createContext<AppState>(null);

export function useAppState() {
  return React.useContext(Context);
}

type Props = {
  state: AppState;
  children: React.ReactNode;
};

export function AppStateProvider(props: Props) {
  return <Context.Provider value={props.state}>{props.children}</Context.Provider>;
}
