import * as React from 'react';

import { Button } from 'src/components/Button';
import { Roulette } from 'src/components/Roulette';
import * as AppState from 'src/core/AppStateContext';
import * as ClientState from 'src/core/ClientStateContext';
import { CardInternal } from './Card';

export function Game() {
  const [state] = AppState.useAppState();
  const game_command = AppState.useGameCommand();

  const message = (function () {
    switch (true) {
      case state.status === AppState.Status.roulette:
        return <Roulette />;

      case !state.status:
        return (
          <div className="flex w-full flex-row justify-center">
            <Button
              color="green"
              onClick={async () => {
                if (state.turn_is_player && state.game.rules.chaos && !state.game.chaos_card) {
                  // set status to chaos to select chaos card for better search
                  console.debug('🎲 select chaos card');
                  const status = AppState.Status.chaos_select;
                  game_command('status', { status });
                } else {
                  game_command('explore');
                }
              }}
            >
              explore
            </Button>
            <div className="w-2" />
            <Button
              color="red"
              onClick={() => {
                game_command('reset');
              }}
            >
              reset
            </Button>
          </div>
        );

      default:
        // return (
        //   <div>
        //     <Button
        //       onClick={() => {
        //         game_command('set_deck');
        //       }}
        //     >
        //       set_deck
        //     </Button>
        //   </div>
        // );
        return null;
    }
  })();

  return <div className="flex h-32 flex-col justify-center pb-8 text-4xl">{message}</div>;
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
  kind: 'player' | 'computer';
  children: React.ReactNode;
};

function Status(props: Props) {
  const status_classes = ['flex h-32 flex-col justify-start text-4xl'];

  if (props.debug) {
    status_classes.push('border-4 border-yellow-200 border-opacity-50');
  } else {
    status_classes.push('border-4 border-transparent');
  }

  let name;

  (function () {
    switch (props.kind) {
      case 'computer':
        name = props.name || 'Select NPC';
        return status_classes.push('items-start ');
      case 'player':
        name = 'You';
        return status_classes.push('items-end ');
      default:
        return status_classes.push('items-center ');
    }
  })();

  // border-4 border-white
  // border-4 border-yellow-200 border-opacity-50
  return (
    <>
      <div className={status_classes.join(' ')}>
        <Name kind={props.kind} name={name} />
        {props.children}
      </div>
      <div className="h-8 w-8" />
    </>
  );
}

type NameProps = {
  kind: 'player' | 'computer' | 'game';
  name: string;
};

function Name(props: NameProps) {
  if (props.kind === 'game') {
    return null;
  }

  const name_classes = ['text-4xl font-bold uppercase'];

  (function () {
    switch (props.kind) {
      case 'computer':
        return name_classes.push('text-red-500');
      case 'player':
        return name_classes.push('text-blue-500');
      default:
        return;
    }
  })();

  if (props.kind === 'player') {
    return (
      <SelectDeck>
        <div className={name_classes.join(' ')}>{props.name}</div>
      </SelectDeck>
    );
  }

  return (
    <MaybeEditName {...props}>
      <div className={name_classes.join(' ')}>{props.name}</div>
    </MaybeEditName>
  );
}

type SelectDeckProps = {
  children: React.ReactNode;
};

type Choices = Array<
  | {
      type: 'text';
      value: 'create' | 'cancel';
      data: string;
    }
  | {
      type: 'deck';
      value: number;
      data: [string, string, string, string, string];
    }
>;
function SelectDeck(props: SelectDeckProps) {
  const [state] = AppState.useAppState();
  const command = AppState.useCommand();
  const game_command = AppState.useGameCommand();

  const [create_deck, set_create_deck] = React.useState<Array<string>>([]);
  const [card_results, set_card_results] = React.useState<Array<AppState.CardJson>>([]);
  const [creating, set_creating] = React.useState(false);

  const deck_choices = (function init_deck_choices() {
    const choices: Choices = [];

    for (let i = 0; i < state.persist_data.deck_list.length; i++) {
      const deck = state.persist_data.deck_list[i];
      const data = deck.cards;
      choices.push({ type: 'deck', value: i, data });
    }

    choices.push({ type: 'text', value: 'create', data: 'Create new deck...' });
    choices.push({ type: 'text', value: 'cancel', data: 'Cancel' });

    return choices;
  })();

  // console.debug('[SelectDeck]', { deck_choices });

  const [editing, set_editing] = React.useState(false);

  function handle_edit() {
    set_editing(true);
  }

  function handle_select_deck(value) {
    console.debug('handle_select_deck', { value });

    set_editing(false);

    switch (true) {
      case value === 'create':
        return set_creating(true);

      case typeof value === 'number': {
        const index = value;
        return game_command('set_deck', { index });
      }

      case value === 'cancel':
      default:
      // no-op
    }
  }

  async function handle_card_search(event) {
    const search = event.currentTarget.value;
    const results = (await command('search_card', { search })) as Array<AppState.CardJson>;
    set_card_results(results);
    // console.debug('handle_card_search', { search, results });
  }

  function handle_add_card(card) {
    // console.debug('handle_add_card', { card });
    set_create_deck((current_deck) => {
      const next_deck = new Set(current_deck);
      next_deck.add(card.id);
      return Array.from(next_deck);
    });
  }

  function handle_remove_card(id) {
    // console.debug('handle_remove_card', { id });
    set_create_deck((current_deck) => {
      const next_deck = new Set(current_deck);
      next_deck.delete(id);
      return Array.from(next_deck);
    });
  }

  if (creating) {
    return (
      <div className="relative z-20 w-full">
        <div className="absolute rounded-md bg-neutral-900 p-16" style={{ width: 360 }}>
          <div className="flex justify-end">
            <button
              className=""
              onClick={() => {
                set_creating(false);
                set_card_results([]);
              }}
            >
              ❌
            </button>
          </div>
          <div className="flex w-full flex-col items-start">
            <div className="mb-4 flex flex-row flex-wrap items-center justify-center">
              {create_deck.map((id) => {
                return (
                  // eslint-disable-next-line react/jsx-key
                  <button onClick={() => handle_remove_card(id)}>
                    <CardInternal id={id} image_id={+id} owner="player" />
                  </button>
                );
              })}
            </div>

            {create_deck.length === 5 ? (
              <div className="mt-8 flex w-full justify-center">
                <Button
                  className="w-full"
                  onClick={() => {
                    const cards = create_deck;
                    game_command('save_deck', { cards });
                    set_creating(false);
                    set_create_deck([]);
                    set_card_results([]);
                  }}
                >
                  Save
                </Button>
              </div>
            ) : (
              <>
                <input
                  autoFocus
                  className="w-full py-4 px-8 text-black"
                  onChange={handle_card_search}
                  placeholder="Enter a card name..."
                />

                <div className="flex w-full flex-col items-start">
                  {card_results.slice(0, 3).map((card) => {
                    // disabling key improves performance
                    return (
                      // eslint-disable-next-line react/jsx-key
                      <button
                        onClick={() => handle_add_card(card)}
                        className="flex w-full flex-nowrap items-start whitespace-pre py-4"
                      >
                        <div className="">
                          <CardInternal id={card.id} image_id={+card.id} owner="player" />
                        </div>

                        <div className="flex justify-start overflow-hidden overflow-ellipsis">
                          <div className="h-8 w-8 min-w-max" />
                          <span className="overflow-hidden overflow-ellipsis">{card.name}</span>
                        </div>
                      </button>
                    );
                  })}
                </div>
              </>
            )}
          </div>
        </div>
      </div>
    );
  }

  if (editing) {
    return (
      <div className="relative z-20 w-full">
        <div className="absolute min-w-full rounded-md bg-neutral-900 p-16">
          <div className="flex w-full flex-col items-start">
            {deck_choices.map((deck_choice) => {
              const content = (function deck_choice_content() {
                switch (deck_choice.type) {
                  case 'deck':
                    return (
                      <div className="flex flex-row flex-nowrap">
                        {deck_choice.data.map((id) => {
                          return <CardInternal key={id} id={id} image_id={+id} owner="player" />;
                        })}
                      </div>
                    );
                  case 'text':
                  default:
                    return deck_choice.data;
                }
              })();

              return (
                <button
                  key={deck_choice.value}
                  onClick={() => handle_select_deck(deck_choice.value)}
                  className="flex w-full items-start py-4"
                >
                  {content}
                </button>
              );
            })}
          </div>
        </div>
      </div>
    );
  }

  return (
    <button className="flex flex-row items-center" onClick={handle_edit}>
      {props.children}
      <span className="h-4 w-4" />
      {'✏️'}
    </button>
  );
}

type MaybeEditNameProps = NameProps & {
  children: React.ReactNode;
};

function MaybeEditName(props: MaybeEditNameProps) {
  const [editing, set_editing] = React.useState(false);
  const [results, set_results] = React.useState<Array<AppState.Npc>>([]);
  const command = AppState.useCommand();
  const game_command = AppState.useGameCommand();

  function handle_click() {
    set_editing(true);
  }

  // console.debug('[EditName]', { search, results, props });

  async function handle_change(event) {
    const search = event.currentTarget.value;
    if (search.length >= 2) {
      const results = (await command('search_npc', { search })) as Array<AppState.Npc>;
      // console.debug('handle_change', { search, results });
      set_results(results);
    } else {
      set_results([]);
    }
  }

  if (editing) {
    return (
      <>
        <input
          autoFocus
          className="w-full py-4 px-8 text-black"
          onChange={handle_change}
          placeholder="Enter an NPC name..."
        />
        <div className="relative z-20 w-full">
          {!results.length ? null : (
            <div className="absolute w-full rounded-md bg-neutral-900 p-16">
              <div className="flex w-full flex-col items-start">
                {results.slice(0, 10).map((result) => {
                  function handle_result_click() {
                    // console.debug('handle_result_click', result);
                    const search = result.name;
                    game_command('set_npc', { search });
                    set_results([]);
                    set_editing(false);
                  }

                  // removing key improves performance
                  return (
                    // eslint-disable-next-line react/jsx-key
                    <button
                      key={result.id}
                      onClick={handle_result_click}
                      className="w-full overflow-hidden overflow-ellipsis whitespace-nowrap py-4 text-left uppercase"
                    >
                      {result.name}
                    </button>
                  );
                })}
              </div>
            </div>
          )}
        </div>
      </>
    );
  }

  return (
    <button className="flex flex-row items-center" onClick={handle_click}>
      {props.children}
      <span className="h-4 w-4" />
      {'✏️'}
    </button>
  );
}
