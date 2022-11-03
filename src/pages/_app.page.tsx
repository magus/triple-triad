import * as React from 'react';
import { type AppProps } from 'next/app';
import * as TauriGlobalShortcut from '@tauri-apps/api/globalShortcut';
import { isTauriApp } from 'src/core/isTauriApp';

// load global styles
import './global.css';

export default function MyApp({ Component, pageProps }: AppProps) {
  // setup global event listeners here
  // global event listeners are active even when window isn't focused
  // prefer normal keyboard listeners instead for window shortcuts
  // such as restart or quit
  // forcefully route to specific page, refresh state, etc.

  // const default_zoom = 1.0;
  // const [zoom, set_zoom] = React.useState(default_zoom);
  // const inc_zoom = (sign: -1 | 1) => (z: number) => +Math.max(0.4, Math.min(0.8, z + 0.1 * sign)).toFixed(2);

  React.useEffect(() => {
    if (!isTauriApp()) return;

    // const TauriWindow = require('@tauri-apps/api/window');
    // console.debug(TauriWindow);
    // window['TauriWindow'] = TauriWindow;

    // setup zoom level and window size in array
    // selection becomes index in array instead of raw zoom level
    // set window size for each zoom level to match scale
    // e.g. large scale, large window ... small scale, small window

    // TauriWindow.appWindow.setSize(new TauriWindow.LogicalSize(800, 600));

    async function run() {
      // https://tauri.app/v1/api/js/globalShortcut
      // await TauriGlobalShortcut.unregisterAll();
      // await TauriGlobalShortcut.unregister('CommandOrControl+M');
      // await TauriGlobalShortcut.unregister('CommandOrControl+0');
      // await TauriGlobalShortcut.unregister('CommandOrControl+-');
      // await TauriGlobalShortcut.unregister('CommandOrControl+=');
      // TauriWindow.register('CommandOrControl+M', () => TauriWindow.appWindow.minimize());
      // TauriWindow.register('CommandOrControl+0', () => set_zoom(default_zoom));
      // TauriWindow.register('CommandOrControl+-', () => set_zoom(inc_zoom(-1)));
      // TauriWindow.register('CommandOrControl+=', () => set_zoom(inc_zoom(+1)));
    }

    run();
  }, []);

  return (
    <div data-tauri-drag-region className="h-full w-full">
      <div
        className="data-tauri-drag-region flex h-full w-full origin-center items-center"
        // style={{ transform: `scale(${zoom})` }}
      >
        <Component {...pageProps} />
      </div>
    </div>
  );
}
