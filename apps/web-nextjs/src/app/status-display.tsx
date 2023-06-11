import type { UseMutationResult } from "@tanstack/react-query";

import type { ShortenResponse } from "api";

export type StatusDisplayProps = UseMutationResult<
  ShortenResponse,
  unknown,
  string,
  unknown
>;

export const StatusDisplay = ({
  isLoading,
  isError,
  error,
  isSuccess,
  data,
}: StatusDisplayProps) => {
  return (
    <>
      {isLoading && <p>Loading...</p>}
      {isSuccess && (
        <p>
          <a href={`/${data.id}`} className="underline">
            BASE_URL/{data.id}
          </a>
        </p>
      )}
      {isError && <p>Error: ${JSON.stringify(error, null, 2)}</p>}
    </>
  );
};
