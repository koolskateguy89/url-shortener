import type { VoidComponent } from "solid-js";
import { useSearchParams } from "solid-start";

import type { Error } from "api";

const ErrorPage: VoidComponent = () => {
  // TODO: include string while still keeping autocomplete
  const [searchParams] = useSearchParams<{ id: string; cause?: Error }>();

  return (
    <main>
      <h1>Error</h1>
      <pre>ID = {searchParams.id}</pre>
      <pre>searchParams = {JSON.stringify(searchParams, null, 2)}</pre>
    </main>
  );
};

export default ErrorPage;
