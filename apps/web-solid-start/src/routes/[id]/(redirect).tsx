import type { VoidComponent } from "solid-js";
import { type RouteDataArgs, createRouteData, redirect } from "solid-start";

import type { routeData as idRouteData } from "../[id]";

// Uses route data from [id] parent layout
export function routeData({ data }: RouteDataArgs<typeof idRouteData>) {
  // Will only be called once data from [id] is ready
  return createRouteData(
    ({ url }) => {
      throw redirect(url);
    },
    {
      key: data,
    }
  );
}

/**
 * Is not meant to display anything.
 */
const RedirectPage: VoidComponent = () => null;

export default RedirectPage;
