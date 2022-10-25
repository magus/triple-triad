import { Html, Head, Main, NextScript, type DocumentProps } from 'next/document';

// https://nextjs.org/docs/advanced-features/custom-document

export default function Document(props: DocumentProps) {
  return (
    <Html lang="en" className="color-black">
      <Head />
      <body>
        <Main />
        <NextScript />
      </body>
    </Html>
  );
}
