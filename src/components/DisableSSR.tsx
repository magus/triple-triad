import dynamic from 'next/dynamic';
import React from 'react';

export const DisableSSR = dynamic(() => Promise.resolve(InternalDisableSSR), {
  ssr: false,
});

function InternalDisableSSR(props) {
  return <React.Fragment>{props.children}</React.Fragment>;
}
