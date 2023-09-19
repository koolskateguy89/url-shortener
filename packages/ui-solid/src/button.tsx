import type { Component, JSX } from "solid-js";
import { mergeProps, splitProps } from "solid-js";
import { Button as KobalteButton } from "@kobalte/core";

import { type ButtonVariantProps, buttonVariants } from "ui-core/button";
import { cn } from "ui-core/utils";

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
  ]);

  const className = () =>
    cn(
      buttonVariants({
        variant: localProps.variant,
        size: localProps.size,
        class: localProps.class,
      }),
    );

  return <KobalteButton.Root class={className()} {...otherProps} />;
};
