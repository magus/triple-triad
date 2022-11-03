import * as React from 'react';
import { CSS } from '@dnd-kit/utilities';
import { useDraggable } from '@dnd-kit/core';

type Props = {
  id: string;
  children: React.ReactNode;
  className: string;
  data?: any;
};

export function Draggable(props: Props) {
  const { isDragging, attributes, listeners, setNodeRef, transform } = useDraggable({
    id: props.id,
    data: props.data,
  });

  const style = {
    zIndex: undefined,
    transform: CSS.Translate.toString(transform),
  };

  if (isDragging) {
    style.zIndex = 9999;
  }

  return (
    <button ref={setNodeRef} style={style} className={props.className} {...listeners} {...attributes}>
      {props.children}
    </button>
  );
}
