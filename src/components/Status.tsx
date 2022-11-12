import * as React from 'react';

import { Button } from 'src/components/Button';
import * as AppState from 'src/core/AppStateContext';

export function Game() {
  const [state] = AppState.useAppState();

  const message = (function () {
    switch (state.status) {
      case AppState.Status.chaos_select:
        return (
          <span>
            Select the card randomly selected by <span className="font-bold">Chaos</span>
          </span>
        );
      case AppState.Status.all_open:
        return (
          <span>
            Select the cards revealed by <span className="font-bold">All Open</span>
            <Button
              onClick={() => {
                console.debug('finalize all_open selection');
              }}
            >
              Confirm
            </Button>
          </span>
        );
      default:
        return null;
    }
  })();

  return <Status kind="game">{message}</Status>;
}

export function Player() {
  const [state] = AppState.useAppState();

  const message = (function () {
    switch (state.status) {
      case AppState.Status.chaos_select:
        return (
          <span>
            Select the card randomly selected by <span className="font-bold">Chaos</span>
          </span>
        );
      case AppState.Status.all_open:
        return (
          <span>
            Select the cards revealed by <span className="font-bold">All Open</span>
            <Button
              onClick={() => {
                console.debug('finalize all_open selection');
              }}
            >
              Confirm
            </Button>
          </span>
        );
      default:
        return null;
    }
  })();

  return <Status kind="player">{message}</Status>;
}

export function Computer() {
  const [state] = AppState.useAppState();

  const name = state?.npc?.name;

  const message = (function () {
    switch (state.status) {
      case AppState.Status.chaos_select:
        return (
          <span>
            Select the card randomly selected by <span className="font-bold">Chaos</span>
          </span>
        );
      case AppState.Status.all_open:
        return (
          <div className="flex flex-row">
            <span>
              Select the cards revealed by <span className="font-bold">All Open</span>
            </span>
            <Button
              onClick={() => {
                console.debug('finalize all_open selection');
              }}
            >
              Confirm
            </Button>
          </div>
        );
      default:
        return null;
    }
  })();

  return (
    <Status name={name} kind="computer">
      {message}
    </Status>
  );
}

type Props = {
  debug?: boolean;
  name?: string;
  kind: 'player' | 'computer' | 'game';
  children: React.ReactNode;
};

function Status(props: Props) {
  const classNames = ['flex h-32 flex-col justify-start text-4xl'];

  if (props.debug) {
    classNames.push('border-4 border-yellow-200 border-opacity-50');
  } else {
    classNames.push('border-4 border-transparent');
  }

  let name;

  (function () {
    switch (props.kind) {
      case 'computer':
        name = props.name;
        return classNames.push('items-start text-red-500');
      case 'player':
        name = 'You';
        return classNames.push('items-end text-blue-500');
      case 'game':
      default:
        return classNames.push('items-center text-white');
    }
  })();

  // border-4 border-white
  // border-4 border-yellow-200 border-opacity-50
  return (
    <>
      <div className={classNames.join(' ')}>
        {!name ? null : <div className="text-4xl font-bold uppercase">{name}</div>}

        {props.children}
      </div>
      <div className="h-8 w-8" />
    </>
  );
}
