import type { Error } from "api";

export default function ErrorPage({
  searchParams,
}: {
  searchParams: {
    id: string;
    cause?: Error | (string & {});
  };
}) {
  return (
    <main>
      <h1>Error</h1>
      <pre>ID = {searchParams.id}</pre>
      <pre>searchParams = {JSON.stringify(searchParams, null, 2)}</pre>
    </main>
  );
}
