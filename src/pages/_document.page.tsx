import { Html, Head, Main, NextScript, type DocumentProps } from 'next/document';

// https://nextjs.org/docs/advanced-features/custom-document

export default function Document(props: DocumentProps) {
  return (
    <Html
      lang="en"
      className="text-rendering-legibility cursor-default select-none overflow-hidden bg-black font-body text-base font-normal text-gray-100 antialiased"
    >
      <Head />
      <body>
        <Main />
        <NextScript />
      </body>
    </Html>
  );
}
