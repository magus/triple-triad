import type { AppProps } from 'next/app';

// load global styles
import '../global.css';
import '../App.css';

// This default export is required in a new `pages/_app.js` file.
export default function MyApp({ Component, pageProps }: AppProps) {
  // setup global event listeners here
  // such as restart or quit
  // forcefully route to specific page, refresh state, etc.

  return <Component {...pageProps} />;
}
