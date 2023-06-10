import type { ComponentProps } from "react";
import { Loader2 } from "lucide-react";

import { cn } from "ui-core";

export interface LoadingSpinnerProps extends ComponentProps<typeof Loader2> {}

export const LoadingSpinner = (props: LoadingSpinnerProps) => {
  return (
    <Loader2
      {...props}
      className={cn("h-4 w-4 animate-spin", props.className)}
    />
  );
};
