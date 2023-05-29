import type { Component, JSX } from "solid-js";
import { mergeProps, splitProps, Show } from "solid-js";

import { type ButtonVariantProps, buttonVariants } from "ui-core/button";
import { cn } from "ui-core/utils";
import { type SlotProps, Slot } from "./slot";

export interface ButtonProps
  extends JSX.ButtonHTMLAttributes<HTMLButtonElement>,
    ButtonVariantProps {
  asChild?: boolean;
}

export const Button: Component<ButtonProps> = (_props) => {
  const merged = mergeProps({ asChild: false }, _props);
  const [localProps, otherProps] = splitProps(merged, [
    "class",
    "variant",
    "size",
    "asChild",
  ]);

  const className = () =>
    cn(
      buttonVariants({
        variant: localProps.variant,
        size: localProps.size,
        class: localProps.class,
      })
    );

  return (
    <Show
      when={localProps.asChild}
      fallback={<button class={className()} {...otherProps} />}
    >
      <Slot class={className()} {...(otherProps as SlotProps)} />
    </Show>
  );
};
