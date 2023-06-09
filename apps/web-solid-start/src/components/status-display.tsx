import { type VoidComponent, Match, Switch } from "solid-js";
import { A } from "solid-start";

import type { ActionStatus } from "~/routes/(home)";

export interface StatusDisplayProps extends ActionStatus {}

export const StatusDisplay: VoidComponent<StatusDisplayProps> = (props) => {
  return (
    <p>
      <Switch>
        <Match when={props.pending}>Loading...</Match>
        <Match when={props.result} keyed>
          {({ id }) => (
            <A href={`/${id}`} class="underline">
              BASE_URL/{id}
            </A>
          )}
        </Match>
        <Match when={props.error as unknown} keyed>
          {(error) => `Error: ${JSON.stringify(error, null, 2)}`}
        </Match>
      </Switch>
    </p>
  );
};
