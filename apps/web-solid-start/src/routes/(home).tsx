import { type VoidComponent, Show } from "solid-js";
import { createRouteAction } from "solid-start";
import type { RouteAction } from "solid-start/data/createRouteAction";
import { z } from "zod";

import { type ShortenResponse, api } from "api";
import { Button, Input, LoadingSpinner } from "ui";
import { StatusDisplay } from "~/components/status-display";

const formDataSchema = z.object({
  url: z.string().url(),
});

export type ActionStatus = RouteAction<FormData, ShortenResponse>[0];

const Home: VoidComponent = () => {
  const [shortening, { Form }] = createRouteAction(
    async (formdata: FormData) => {
      const wrappedUrl = formDataSchema.safeParse(Object.fromEntries(formdata));

      if (!wrappedUrl.success) {
        alert("Invalid URL");
        return;
      }

      const { url } = wrappedUrl.data;

      return await api.shorten(url);
    }
  );
  shortening satisfies ActionStatus;

  return (
    <main class="flex h-screen flex-col items-center justify-center space-y-4">
      <StatusDisplay {...shortening} />

      <Form class="flex flex-col items-center space-y-2">
        <Input
          type="url"
          name="url"
          placeholder="Url"
          disabled={shortening.pending}
        />
        <Button type="submit" disabled={shortening.pending}>
          <Show when={shortening.pending}>
            <LoadingSpinner />
          </Show>
          Shorten
        </Button>
      </Form>
    </main>
  );
};

export default Home;
