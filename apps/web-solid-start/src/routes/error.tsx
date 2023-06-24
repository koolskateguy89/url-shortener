import type { VoidComponent } from "solid-js";
import { useSearchParams } from "solid-start";

import type { Error } from "api";

const ErrorPage: VoidComponent = () => {
  const [searchParams] = useSearchParams<{
    id: string;
    cause?: Error | (string & {});
  }>();

  return (
    <main>
      <h1>Error</h1>
      <pre>ID = {searchParams.id}</pre>
      <pre>searchParams = {JSON.stringify(searchParams, null, 2)}</pre>
    </main>
  );
};

export default ErrorPage;
