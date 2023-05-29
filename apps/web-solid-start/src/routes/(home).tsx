import { type VoidComponent, Show } from "solid-js";
import { createRouteAction } from "solid-start";
import { Loader2 } from "lucide-solid";
import { z } from "zod";

import { Button, Input } from "ui";

const formDataSchema = z.object({
  url: z.string().url(),
});

// TODO: display error in UI
// TODO: use API package (TODO) - will need to store API URL in env

const Home: VoidComponent = () => {
  const [shortening, { Form }] = createRouteAction(
    async (formdata: FormData, { fetch }) => {
      const wrappedUrl = formDataSchema.safeParse(Object.fromEntries(formdata));

      if (!wrappedUrl.success) {
        alert("Invalid URL");
        return;
      }

      const { url } = wrappedUrl.data;

      const res = await fetch("http://localhost:8000/", {
        method: "POST",
        body: JSON.stringify({ url }),
        headers: {
          "Content-Type": "application/json",
        },
      });
      const result = (await res.json()) as { url: string };

      return result.url;
    }
  );

  return (
    <main class="flex h-screen flex-col items-center justify-center space-y-4">
      <Form class="flex flex-col items-center space-y-2">
        <Input name="url" disabled={shortening.pending} />
        <Button type="submit" disabled={shortening.pending}>
          <Show when={shortening.pending}>
            <Loader2 class="mr-2 h-4 w-4 animate-spin" />
          </Show>
          Shorten
        </Button>
      </Form>
      <div>shortening = {JSON.stringify(shortening, null, 2)}</div>
    </main>
  );
};

export default Home;
