import * as React from 'react';
import dynamic from 'next/dynamic';

export const DisableSSR = dynamic(() => Promise.resolve(InternalDisableSSR), {
  ssr: false,
});

function InternalDisableSSR(props) {
  return <React.Fragment>{props.children}</React.Fragment>;
}
