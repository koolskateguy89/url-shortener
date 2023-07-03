import { type VoidComponent, createResource } from "solid-js";

import { api } from "api";
import { Button, LoadingSpinner } from "ui";

export const WhoAmI: VoidComponent = () => {
  const [whoami, { refetch }] = createResource(async () => {
    const me = await api.whoami();
    console.log("i am:", me);
    return me;
  });

  const handleRefetch = () => {
    void refetch();
  };

  return (
    <>
      <pre>
        me = {whoami.loading && <LoadingSpinner class="mr-2 inline" />}
        <code>{JSON.stringify(whoami(), null, 2)}</code>
      </pre>

      <Button onclick={handleRefetch} variant="destructive">
        Refetch
      </Button>
    </>
  );
};
