import { type VoidComponent, Show } from "solid-js";
import {
  type RouteDataArgs,
  createRouteData,
  redirect,
  useParams,
  useRouteData,
  Title,
} from "solid-start";

import { api, errorUrl } from "api";

export function routeData({ params }: RouteDataArgs) {
  return createRouteData(
    async ({ id }) => {
      const res = await api.getStats(id);

      if (!res.success) throw redirect(errorUrl(id, "NotFound"));

      console.log("(server) stats =", res.data);

      return res.data;
    },
    {
      key: () => ({
        id: params.id,
        key: "stats",
      }),
    }
  );
}

const StatsPage: VoidComponent = () => {
  const stats = useRouteData<typeof routeData>();

  const params = useParams<{ id: string }>();

  console.log("stats =", stats());

  return (
    <Show when={stats()} fallback={"Failed to fetch stats"} keyed>
      {({ url, num_hits }) => (
        <main>
          <Title>{params.id} Stats</Title>
          <h1>Stats</h1>
          <p>URL: {url}</p>
          <p>Num hits: {num_hits}</p>
        </main>
      )}
    </Show>
  );
};

export default StatsPage;
