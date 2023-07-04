import type { VoidComponent } from "solid-js";
import {
  type RouteDataArgs,
  createRouteData,
  redirect,
  Outlet,
} from "solid-start";

import { type Error, api } from "api";

export function routeData({ params }: RouteDataArgs) {
  return createRouteData(
    async ({ id }) => {
      const exists = await api.idExists(id);

      // FIXME: redirect is not working
      if (!exists)
        throw redirect(`/error?id=${id}&cause=${"NotFound" satisfies Error}`);
    },
    {
      key: () => ({
        id: params.id,
        key: "layout",
      }),
    }
  );
}

/**
 * Is not meant to add any UI.
 */
const Wrapper: VoidComponent = () => <Outlet />;

export default Wrapper;
