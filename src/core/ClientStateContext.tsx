import * as React from 'react';
import * as AppState from 'src/core/AppStateContext';

export type ClientState = {
  scale: number;
  explore_result_index: number;
  all_open_select: Set<number>;
  three_open_select: Set<number>;
};

// hook tuple because we return React.useState
type ContextValue = NullContextValue | ValidContextValue;
type ValidContextValue = [ClientState, React.Dispatch<React.SetStateAction<ClientState>>];
type NullContextValue = [null, React.Dispatch<React.SetStateAction<ClientState>>];

const Context = React.createContext<null | ContextValue>(null);

type Props = {
  children: React.ReactNode;
};

function DefaultClientState() {
  return {
    scale: 1,
    explore_result_index: 0,
    all_open_select: new Set([]),
    three_open_select: new Set([]),
  };
}

export function ClientStateProvider(props: Props) {
  const state = React.useState<ClientState>(DefaultClientState());

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

export function useReset() {
  const [, set_state] = useClientState();

  function reset() {
    set_state((current_state) => {
      const next_state = { ...current_state };

      next_state.all_open_select = new Set([]);
      next_state.three_open_select = new Set([]);

      return next_state;
    });
  }

  return reset;
}
export function useAllOpen() {
  const [state, set_state] = useClientState();
  const [app_state] = AppState.useAppState();

  let guaranteed_card_count = 0;

  if (app_state.npc?.cards) {
    for (const card of app_state.npc.cards) {
      if (card.is_guaranteed) {
        guaranteed_card_count++;
      }
    }
  }

  let select_count = 5 - guaranteed_card_count;

  const selected = state.all_open_select;
  const done = selected.size === select_count;

  function toggle(index) {
    set_state((current_state) => {
      const next_state = { ...current_state };
      const set = new Set(Array.from(next_state.all_open_select));

      if (set.has(index)) {
        set.delete(index);
      } else {
        if (!done) {
          set.add(index);
        }
      }

      next_state.all_open_select = set;

      return next_state;
    });
  }

  return { done, selected, toggle };
}

export function useThreeOpen() {
  const [state, set_state] = useClientState();
  const [app_state] = AppState.useAppState();

  const selected = state.three_open_select;
  const done = selected.size === 3;

  function toggle(index) {
    set_state((current_state) => {
      const next_state = { ...current_state };
      const set = new Set(Array.from(next_state.three_open_select));

      if (set.has(index)) {
        set.delete(index);
      } else {
        if (!done) {
          set.add(index);
        }
      }

      next_state.three_open_select = set;

      return next_state;
    });
  }

  return { done, selected, toggle };
}
