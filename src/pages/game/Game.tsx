import * as React from 'react';
import { DisableSSR } from 'src/components/DisableSSR';
import { GameInternal } from './GameInternal';

export function Game() {
  return (
    <DisableSSR>
      <GameInternal />
    </DisableSSR>
  );
}
