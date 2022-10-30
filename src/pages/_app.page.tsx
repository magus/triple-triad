import * as React from 'react';
import { type AppProps } from 'next/app';
import * as TauriGlobalShortcut from '@tauri-apps/api/globalShortcut';

// load global styles
import './global.css';

export default function MyApp({ Component, pageProps }: AppProps) {
  // setup global event listeners here
  // such as restart or quit
  // forcefully route to specific page, refresh state, etc.
  const default_zoom = 0.6;
  const [zoom, set_zoom] = React.useState(default_zoom);
  const inc_zoom = (sign: -1 | 1) => (z: number) => +Math.max(0.4, Math.min(0.8, z + 0.1 * sign)).toFixed(2);

  React.useEffect(() => {
    const TauriWindow = require('@tauri-apps/api/window');

    // setup zoom level and window size in array
    // selection becomes index in array instead of raw zoom level
    // set window size for each zoom level to match scale
    // e.g. large scale, large window ... small scale, small window
    console.debug(TauriWindow);
    window.TauriWindow = TauriWindow;
    // TauriWindow.appWindow.setSize(new TauriWindow.LogicalSize(800, 600));

    async function run() {
      // https://tauri.app/v1/api/js/globalShortcut
      await TauriGlobalShortcut.unregisterAll();
      await TauriGlobalShortcut.unregister('CommandOrControl+M');
      await TauriGlobalShortcut.unregister('CommandOrControl+0');
      await TauriGlobalShortcut.unregister('CommandOrControl+-');
      await TauriGlobalShortcut.unregister('CommandOrControl+=');

      TauriGlobalShortcut.register('CommandOrControl+M', () => TauriWindow.appWindow.minimize());

      TauriGlobalShortcut.register('CommandOrControl+0', () => set_zoom(default_zoom));
      TauriGlobalShortcut.register('CommandOrControl+-', () => set_zoom(inc_zoom(-1)));
      TauriGlobalShortcut.register('CommandOrControl+=', () => set_zoom(inc_zoom(+1)));
    }

    run();
  }, []);

  return (
    <div data-tauri-drag-region className="h-full w-full">
      <div
        className="data-tauri-drag-region flex h-full w-full origin-center content-center items-center"
        style={{ transform: `scale(${zoom})` }}
      >
        <Component {...pageProps} />
      </div>
    </div>
  );
}
