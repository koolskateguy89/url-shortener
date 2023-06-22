import { type VoidComponent, Show } from "solid-js";
import { type RouteDataArgs, createRouteData, useRouteData } from "solid-start";

import { type StatsResponse, api } from "api";

export function routeData({ params }: RouteDataArgs) {
  return createRouteData(
    async ({ id }) => {
      // Guaranteed to not fail because of validation at layout level
      const stats = (await api.getStats(id)) as StatsResponse;

      console.log("(server) stats =", stats);

      return stats;
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

  console.log("stats =", stats());

  return (
    <Show when={stats()} fallback={"Failed to fetch stats"} keyed>
      {({ url, num_hits }) => (
        <main>
          <h1>Stats</h1>
          <p>URL: {url}</p>
          <p>Num hits: {num_hits}</p>
        </main>
      )}
    </Show>
  );
};

export default StatsPage;
