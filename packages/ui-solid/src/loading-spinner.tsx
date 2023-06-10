import type { ComponentProps, VoidComponent } from "solid-js";
import { Loader2 } from "lucide-solid";

import { cn } from "ui-core";

export interface LoadingSpinnerProps extends ComponentProps<typeof Loader2> {}

export const LoadingSpinner: VoidComponent<LoadingSpinnerProps> = (props) => {
  return <Loader2 {...props} class={cn("h-4 w-4 animate-spin", props.class)} />;
};
