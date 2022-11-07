import * as React from 'react';
import { motion } from 'framer-motion';
import { CSS } from '@dnd-kit/utilities';
import { useDraggable } from '@dnd-kit/core';

type Props = {
  id: string;
  children: React.ReactNode;
  className?: string;
  data?: any;
};

export function Draggable(props: Props) {
  const persist_style = React.useRef(undefined);

  const { isDragging, attributes, listeners, setNodeRef, transform } = useDraggable({
    id: props.id,
    data: props.data,
  });

  const animate = {
    scale: 1,
  };

  const style = {
    zIndex: undefined,
    transform: CSS.Translate.toString(transform),
  };

  if (isDragging) {
    // elevate active drag element above other elements
    // normal z-index is 10, multiple by 10 to get 100
    style.zIndex = 100;

    animate.scale = 1.2;

    // capture ongoing drag so we can persist it when it ends
    persist_style.current = style;
  }

  if (persist_style.current) {
    Object.assign(style, persist_style.current);
  }

  return (
    <button ref={setNodeRef} style={style} className={props.className} {...listeners} {...attributes}>
      <motion.div animate={animate}>{props.children}</motion.div>
    </button>
  );
}
