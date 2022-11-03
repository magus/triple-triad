import * as React from 'react';
import { useDroppable } from '@dnd-kit/core';

type Props = {
  id: string;
  children: React.ReactNode;
  OverElement: React.ComponentType;
};

export function Droppable(props: Props) {
  const { isOver, setNodeRef } = useDroppable({
    id: props.id,
  });

  return (
    <div ref={setNodeRef} className="relative">
      {props.children}
      {!isOver ? null : <props.OverElement />}
      {/* {props.isOverElement} */}
    </div>
  );
}
