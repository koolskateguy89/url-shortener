import type { Component, JSX } from "solid-js";
import { splitProps } from "solid-js";

import { cn } from "ui-core/utils";

export interface InputProps extends JSX.InputHTMLAttributes<HTMLInputElement> {}

export const Input: Component<InputProps> = (_props) => {
  const [localProps, otherProps] = splitProps(_props, ["class", "type"]);

  return (
    <input
      type={localProps.type}
      class={cn(
        "border-input bg ring-offset-background placeholder:text-muted-foreground focus-visible:ring-ring flex h-10 w-full rounded-md border bg-transparent px-3 py-2 text-sm file:border-0 file:bg-transparent file:text-sm file:font-medium focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50",
        localProps.class
      )}
      {...otherProps}
    />
  );
};
