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
      <code>
        me = {whoami.loading && <LoadingSpinner class="mr-2 inline" />}
        {JSON.stringify(whoami(), null, 2)}
      </code>

      <Button onclick={handleRefetch} variant="destructive">
        Refetch
      </Button>
    </>
  );
};
