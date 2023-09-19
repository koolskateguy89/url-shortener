import { type VoidComponent, Match, Switch } from "solid-js";
import { A } from "solid-start";

import type { ActionStatus } from "~/routes/(home)";

export interface StatusDisplayProps extends ActionStatus {}

export const StatusDisplay: VoidComponent<StatusDisplayProps> = (props) => {
  return (
    <Switch>
      <Match when={props.pending}>
        <p>Loading...</p>
      </Match>
      <Match when={props.result} keyed>
        {({ id }) => (
          <p>
            <A href={`/${id}`} class="underline">
              {id}
            </A>
          </p>
        )}
      </Match>
      <Match when={props.error as unknown} keyed>
        {(error) => <p>Error: ${JSON.stringify(error, null, 2)}</p>}
      </Match>
    </Switch>
  );
};
