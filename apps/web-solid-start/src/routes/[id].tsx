import type { VoidComponent } from "solid-js";
import {
  type RouteDataArgs,
  createRouteData,
  redirect,
  Outlet,
} from "solid-start";

import { api } from "api";

export function routeData({ params }: RouteDataArgs) {
  return createRouteData(
    async ({ id }) => {
      const exists = await api.idExists(id);

      if (!exists) throw redirect(`/${id}/error?cause=404`);
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
