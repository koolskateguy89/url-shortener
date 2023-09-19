import type { VoidComponent } from "solid-js";
import { type RouteDataArgs, createRouteData, redirect } from "solid-start";

import { api, errorUrl } from "api";

export function routeData({ params }: RouteDataArgs) {
  return createRouteData(
    async ({ id }) => {
      const res = await api.lengthen(id);

      if (res.success) {
        throw redirect(res.data.url);
      } else {
        throw redirect(errorUrl(id, "NotFound"));
      }
    },
    {
      key: () => ({
        id: params.id,
        key: "redirect",
      }),
    },
  );
}

/**
 * Is not meant to display anything.
 */
const RedirectPage: VoidComponent = () => null;

export default RedirectPage;
