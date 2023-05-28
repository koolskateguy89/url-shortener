// TODO: convert to solid
// https://github.com/radix-ui/primitives/blob/main/packages/react/slot/src/Slot.tsx

import type { JSX, ParentComponent } from "solid-js";

/* -------------------------------------------------------------------------------------------------
 * Slot
 * -----------------------------------------------------------------------------------------------*/

export interface SlotProps extends JSX.HTMLAttributes<HTMLDivElement> {
  children?: JSX.Element;
}

export const Slot: ParentComponent<SlotProps> = (props) => {
  return <>{props.children}</>;
};

/* -------------------------------------------------------------------------------------------------
 * Slottable
 * -----------------------------------------------------------------------------------------------*/

export const Slottable: ParentComponent = (props) => {
  return <>{props.children}</>;
};
