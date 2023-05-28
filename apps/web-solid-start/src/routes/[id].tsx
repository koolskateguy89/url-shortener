import { type VoidComponent, Suspense } from "solid-js";
import {
  type RouteDataArgs,
  createRouteData,
  redirect,
  useRouteData,
} from "solid-start";

export function routeData({ params }: RouteDataArgs) {
  return createRouteData(
    (id) => {
      // TODO: fetch id from backend, then redirect. Else show error
      return 2;
    },
    {
      key: () => params.id,
    }
  );
}

const ErrorPage: VoidComponent = () => {
  const errr = useRouteData<typeof routeData>();

  return (
    <Suspense fallback="LOADINGGGGG">
      <div>
        <h1>Page</h1>
        If you are seeing this, the URL is invalid.
        {errr()}
      </div>
    </Suspense>
  );
};

export default ErrorPage;
