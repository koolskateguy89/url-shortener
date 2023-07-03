"use client";

import { useQuery } from "@tanstack/react-query";

import { api } from "api";
import { Button, LoadingSpinner } from "ui";

export const WhoAmI = () => {
  const whoAmIQuery = useQuery({
    queryKey: ["whoami"],
    queryFn: async () => {
      const me = await api.whoami();
      console.log("i am:", me);
      return me;
    },
  });

  const handleRefetch = () => {
    void whoAmIQuery.refetch();
  };

  return (
    <>
      <pre>
        me ={" "}
        {whoAmIQuery.isFetching && <LoadingSpinner className="mr-2 inline" />}
        <code>{JSON.stringify(whoAmIQuery.data, null, 2)}</code>
      </pre>

      <Button onClick={handleRefetch} variant="destructive">
        Refetch
      </Button>
    </>
  );
};
