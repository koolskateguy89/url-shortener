import Link from "next/link";
import type { UseMutationResult } from "@tanstack/react-query";

import type { api } from "api";

export type StatusDisplayProps = UseMutationResult<
  Awaited<ReturnType<typeof api.shorten>>,
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
          {data.success ? (
            <Link href={`/${data.data.id}`} className="underline">
              BASE_URL/{data.data.id}
            </Link>
          ) : (
            "Error"
          )}
        </p>
      )}
      {isError && <p>Error: ${JSON.stringify(error, null, 2)}</p>}
    </>
  );
};
