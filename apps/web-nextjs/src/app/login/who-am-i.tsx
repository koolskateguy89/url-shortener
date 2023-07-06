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
      <code>
        me ={" "}
        {whoAmIQuery.isFetching && <LoadingSpinner className="mr-2 inline" />}
        {JSON.stringify(whoAmIQuery.data, null, 2)}
      </code>

      <Button onClick={handleRefetch} variant="destructive">
        Refetch
      </Button>
    </>
  );
};
