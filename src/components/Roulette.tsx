import * as React from 'react';

import { Button } from 'src/components/Button';
import * as AppState from 'src/core/AppStateContext';

export function Roulette() {
  const [state] = AppState.useAppState();
  const game_command = AppState.useGameCommand();
  const [rules, set_rules] = React.useState(state.game.rules);

  function handle_confirm() {
    game_command('roulette', { rules });
  }

  return (
    <div className="z-20 flex flex-col items-center">
      <span>
        Select <span className="font-bold">Roulette</span> rules
      </span>

      <div className="h-8 w-8" />

      <div className="rounded-md bg-neutral-900 p-16" style={{ width: 720 }}>
        <div className="mb-16 flex justify-center">
          <Button onClick={handle_confirm}>Confirm</Button>
        </div>

        <div className="flex flex-row flex-wrap justify-center">
          {Object.keys(rules).map((rule) => {
            if (rule === 'roulette') return null;
            if (rule === 'draft') return null;

            const color = rules[rule] ? 'green' : 'gray';

            function handle_click() {
              set_rules((current_rules) => {
                const next_rules = { ...current_rules };
                next_rules[rule] = !next_rules[rule];
                return next_rules;
              });
            }

            return (
              <div key={rule} className="mb-8 mr-8">
                <Button color={color} onClick={handle_click}>
                  {rule}
                </Button>
              </div>
            );
          })}
        </div>
      </div>
    </div>
  );
}
