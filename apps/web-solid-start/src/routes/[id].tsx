import { type VoidComponent, Suspense } from "solid-js";
import {
  type RouteDataArgs,
  createRouteData,
  redirect,
  useRouteData,
} from "solid-start";

// FIXME: this is just not working, the fetch is failing, idk why
export function routeData({ params }: RouteDataArgs) {
  return createRouteData(
    async (id) => {
      console.log("id =", id);

      // TODO: use api pacakge
      const res = await fetch(`http://localhost:8000/api/${id}`, {});
      const result = (await res.json()) as { url: string };

      console.log("result =", result);

      throw redirect(result.url);
      return "wtf";
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
