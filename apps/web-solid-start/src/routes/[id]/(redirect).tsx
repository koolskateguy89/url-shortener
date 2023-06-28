import type { VoidComponent } from "solid-js";
import { type RouteDataArgs, createRouteData, redirect } from "solid-start";

import { type LengthenResponse, api } from "api";

export function routeData({ params }: RouteDataArgs) {
  return createRouteData(
    async ({ id }) => {
      // Guaranteed to not fail because of validation at layout level
      const { url } = (await api.lengthen(id)) as LengthenResponse;
      // FIXME: isn't working when opening the URL in a new tab
      throw redirect(url);
    },
    {
      key: () => ({
        id: params.id,
        key: "redirect",
      }),
    }
  );
}

/**
 * Is not meant to display anything.
 */
const RedirectPage: VoidComponent = () => null;

export default RedirectPage;
