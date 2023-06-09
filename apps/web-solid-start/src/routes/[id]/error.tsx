import type { VoidComponent } from "solid-js";
import { useParams, useSearchParams } from "solid-start";

import type { Error } from "api";

const ErrorPage: VoidComponent = () => {
  const params = useParams<{ id: string }>();

  // TODO: include string while still keeping autocomplete
  const [searchParams] = useSearchParams<{ cause?: Error }>();

  return (
    <main>
      <h1>Page</h1>
      If you are seeing this, the URL is invalid.
      <pre>params = {JSON.stringify(params, null, 2)}</pre>
      <pre>searchParams = {JSON.stringify(searchParams, null, 2)}</pre>
    </main>
  );
};

export default ErrorPage;
