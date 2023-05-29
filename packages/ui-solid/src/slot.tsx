// TODO: convert to solid
// https://github.com/radix-ui/primitives/blob/main/packages/react/slot/src/Slot.tsx

import type { Component, JSX } from "solid-js";

type ElementProps = JSX.HTMLAttributes<HTMLElement>;

/* -------------------------------------------------------------------------------------------------
 * Slot
 * -----------------------------------------------------------------------------------------------*/

export interface SlotProps extends ElementProps {}

export const Slot: Component<SlotProps> = (props) => {
  return <>{props.children}</>;
};

/* -------------------------------------------------------------------------------------------------
 * Slottable
 * -----------------------------------------------------------------------------------------------*/

export interface SlottableProps extends ElementProps {}

export const Slottable: Component<SlottableProps> = (props) => (
  <>{props.children}</>
);
