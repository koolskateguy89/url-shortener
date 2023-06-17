// TODO

import { type StatsResponse, api } from "api";

export default async function StatsPage({
  params,
}: {
  params: {
    id: string;
  };
}) {
  const { url, hits } = (await api.getStats(params.id)) as StatsResponse;

  return (
    <main>
      <h1>Stats</h1>
      <p>URL: {url}</p>
      <p>Hits: {hits}</p>
    </main>
  );
}
