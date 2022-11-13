import * as React from 'react';

import { Button } from 'src/components/Button';
import { Roulette } from 'src/components/Roulette';
import * as AppState from 'src/core/AppStateContext';
import * as ClientState from 'src/core/ClientStateContext';

export function Game() {
  const [state] = AppState.useAppState();

  const message = (function () {
    switch (state.status) {
      case AppState.Status.roulette:
        return <Roulette />;

      default:
        return null;
    }
  })();

  return <Status kind="game">{message}</Status>;
}

export function Player() {
  const [state] = AppState.useAppState();
  const swap = ClientState.useSwap();
  const game_command = AppState.useGameCommand();

  const message = (function () {
    switch (state.status) {
      case AppState.Status.chaos_select:
        return (
          <span>
            Select the card randomly selected by <span className="font-bold">Chaos</span>
          </span>
        );

      case AppState.Status.swap:
        return (
          <div className="flex w-full flex-row justify-between">
            <span>
              Select <span className="font-bold">Swap</span> card
            </span>
            {!swap.done ? null : (
              <Button
                onClick={async () => {
                  const { player, computer } = swap;
                  await game_command('swap', { player, computer });
                }}
              >
                Confirm
              </Button>
            )}
          </div>
        );

      default:
        return null;
    }
  })();

  return <Status kind="player">{message}</Status>;
}

export function Computer() {
  const [state] = AppState.useAppState();
  const all_open = ClientState.useAllOpen();
  const three_open = ClientState.useThreeOpen();
  const swap = ClientState.useSwap();
  const game_command = AppState.useGameCommand();

  const name = state?.npc?.name;

  const message = (function () {
    switch (state.status) {
      case AppState.Status.all_open:
        return (
          <div className="flex w-full flex-row justify-between">
            <span>
              Select the cards revealed by <span className="font-bold">All Open</span>
            </span>
            {!all_open.done ? null : (
              <Button
                onClick={async () => {
                  const cards = Array.from(all_open.selected);
                  await game_command('all_open', { cards });
                }}
              >
                Confirm
              </Button>
            )}
          </div>
        );

      case AppState.Status.three_open:
        return (
          <div className="flex w-full flex-row justify-between">
            <span>
              Select the cards revealed by <span className="font-bold">Three Open</span>
            </span>
            {!three_open.done ? null : (
              <Button
                onClick={async () => {
                  const cards = Array.from(three_open.selected);
                  await game_command('three_open', { cards });
                }}
              >
                Confirm
              </Button>
            )}
          </div>
        );

      case AppState.Status.swap:
        return (
          <div className="flex w-full flex-row justify-between">
            <span>
              Select <span className="font-bold">Swap</span> card
            </span>
            {!swap.done ? null : (
              <Button
                onClick={async () => {
                  const { player, computer } = swap;
                  await game_command('swap', { player, computer });
                }}
              >
                Confirm
              </Button>
            )}
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
  const status_classes = ['flex h-32 flex-col justify-start text-4xl'];
  const name_classes = ['text-4xl font-bold uppercase'];

  if (props.debug) {
    status_classes.push('border-4 border-yellow-200 border-opacity-50');
  } else {
    status_classes.push('border-4 border-transparent');
  }

  let name;

  (function () {
    switch (props.kind) {
      case 'computer':
        name = props.name;
        name_classes.push('text-red-500');
        return status_classes.push('items-start ');
      case 'player':
        name = 'You';
        name_classes.push('text-blue-500');
        return status_classes.push('items-end ');
      case 'game':
      default:
        name_classes.push('text-white');
        return status_classes.push('items-center ');
    }
  })();

  // border-4 border-white
  // border-4 border-yellow-200 border-opacity-50
  return (
    <>
      <div className={status_classes.join(' ')}>
        {!name ? null : <div className={name_classes.join(' ')}>{name}</div>}

        {props.children}
      </div>
      <div className="h-8 w-8" />
    </>
  );
}
