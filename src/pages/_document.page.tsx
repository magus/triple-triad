import { Html, Head, Main, NextScript, type DocumentProps } from 'next/document';

// https://nextjs.org/docs/advanced-features/custom-document

export default function Document(props: DocumentProps) {
  return (
    <Html
      lang="en"
      className="select-none cursor-default font-body text-base font-normal text-rendering-legibility antialiased bg-black text-gray-100"
    >
      <Head />
      <body>
        <Main />
        <NextScript />
      </body>
    </Html>
  );
}
