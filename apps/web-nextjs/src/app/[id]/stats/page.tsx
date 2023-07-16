import { api, errorUrl } from "api";
import { redirect } from "next/navigation";

export default async function StatsPage({
  params,
}: {
  params: {
    id: string;
  };
}) {
  const res = await api.getStats(params.id);

  if (!res.success) throw redirect(errorUrl(params.id, "NotFound"));

  const { url, username, num_hits } = res.data;

  return (
    <main>
      <h1>Stats</h1>
      <p>URL: {url}</p>
      <p>Num hits: {num_hits}</p>
      <p>Username: {username ?? "null"}</p>
    </main>
  );
}
