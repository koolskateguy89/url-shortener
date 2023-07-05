import Link from "next/link";
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
          <Link href={`/${data.id}`} className="underline">
            BASE_URL/{data.id}
          </Link>
        </p>
      )}
      {isError && <p>Error: ${JSON.stringify(error, null, 2)}</p>}
    </>
  );
};
