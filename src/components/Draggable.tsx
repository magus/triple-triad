import * as React from 'react';
import { CSS } from '@dnd-kit/utilities';
import { useDraggable } from '@dnd-kit/core';

type Props = {
  id: string;
  children: React.ReactNode;
  className?: string;
  data?: any;
};

export function Draggable(props: Props) {
  const persist_transform = React.useRef(undefined);

  const { isDragging, attributes, listeners, setNodeRef, transform } = useDraggable({
    id: props.id,
    data: props.data,
  });

  const transform_string = CSS.Translate.toString(transform);

  const style = {
    zIndex: undefined,
    transform: transform_string || persist_transform.current,
  };

  if (isDragging) {
    // capture ongoing drag so we can persist it when it ends
    persist_transform.current = transform_string;

    // elevate active drag element above other elements
    // normal z-index is 10, multiple by 10 to get 100
    style.zIndex = 100;
  }

  return (
    <button ref={setNodeRef} style={style} className={props.className} {...listeners} {...attributes}>
      {props.children}
    </button>
  );
}
