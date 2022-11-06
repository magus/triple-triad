import * as React from 'react';
import { DisableSSR } from 'src/components/DisableSSR';
import { AppStateProvider } from 'src/core/AppStateContext';
import { ClientStateProvider } from 'src/core/ClientStateContext';
import { GameInternal } from './GameInternal';

export function Game() {
  return (
    <DisableSSR>
      <AppStateProvider>
        <ClientStateProvider>
          <GameInternal />
        </ClientStateProvider>
      </AppStateProvider>
    </DisableSSR>
  );
}
