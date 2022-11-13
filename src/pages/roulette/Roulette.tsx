import * as React from 'react';
import Link from 'next/link';

import * as AppState from 'src/core/AppStateContext';

export function Roulette() {
  const [state] = AppState.useAppState();

  const [rules, set_rules] = React.useState(() => {
    const init_rules = {
      ...state.game.rules,
    };

    return init_rules;
  });

  return (
    <div className="container">
      <Link href="/">Confirm</Link>

      {Object.keys(rules).map((rule) => {
        return (
          <div key={rule}>
            {rule} = {rules[rule]}
          </div>
        );
      })}
    </div>
  );
}
