import * as React from 'react';
import { type AppProps } from 'next/app';
import * as TauriGlobalShortcut from '@tauri-apps/api/globalShortcut';

// load global styles
import './global.css';

export default function MyApp({ Component, pageProps }: AppProps) {
  // setup global event listeners here
  // such as restart or quit
  // forcefully route to specific page, refresh state, etc.
  const [zoom, set_zoom] = React.useState(0.6);

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

      const inc_zoom = (inc) => (z) => +Math.max(0.6, Math.min(0.8, z + inc)).toFixed(2);
      TauriGlobalShortcut.register('CommandOrControl+0', () => set_zoom(0.6));
      TauriGlobalShortcut.register('CommandOrControl+-', () => set_zoom(inc_zoom(-0.05)));
      TauriGlobalShortcut.register('CommandOrControl+=', () => set_zoom(inc_zoom(+0.05)));
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
