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
    async (id) => {
      const res = await api.lengthen(id);

      if ("error" in res) {
        const error =
          typeof res.error === "string" ? res.error : res.error.Other;

        throw redirect(`/${id}/error?cause=${error}`);
      }

      return res;
    },
    {
      key: () => params.id,
    }
  );
}

/**
 * Is not meant to add any UI.
 */
const Wrapper: VoidComponent = () => <Outlet />;

export default Wrapper;
