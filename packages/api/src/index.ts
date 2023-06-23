// eslint-disable-next-line @typescript-eslint/no-unsafe-assignment
const API_URL: string =
  // @ts-expect-error Tryna use both Vite and Next
  // eslint-disable-next-line @typescript-eslint/no-unsafe-member-access
  import.meta.env?.VITE_API_URL ?? process.env.NEXT_PUBLIC_API_URL;

if (!API_URL) throw new Error("No API_URL from environment variables");

export interface ShortenResponse {
  id: string;
}

export interface LengthenResponse {
  url: string;
}

export interface StatsResponse {
  url: string;
  num_hits: number;
  /**
   * UTC timestamps
   */
  hits: number[];
}

export type Error = "NotFound" | "InvalidUrl";

export interface ErrorResponse {
  error:
    | Error
    | {
        Other: string;
      };
}

/**
 * Error response should not occur, but if it does, it will be a 500.
 *
 * @param url
 * @returns
 */
async function shorten(url: string): Promise<ShortenResponse> {
  const res = await fetch(API_URL, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ url }),
  });

  return (await res.json()) as ShortenResponse;
}

async function lengthen(
  id: string,
  init: RequestInit = {
    cache: "no-cache",
  }
): Promise<LengthenResponse | ErrorResponse> {
  const res = await fetch(`${API_URL}/${encodeURIComponent(id)}`, init);

  return (await res.json()) as LengthenResponse | ErrorResponse;
}

async function idExists(id: string): Promise<boolean> {
  const res = await fetch(`${API_URL}/${id}/exists`);

  return res.ok;
}

async function getStats(
  id: string,
  init: RequestInit = {
    cache: "no-cache",
  }
): Promise<StatsResponse | ErrorResponse> {
  const res = await fetch(`${API_URL}/${encodeURIComponent(id)}/stats`, init);

  return (await res.json()) as StatsResponse | ErrorResponse;
}

export const api = {
  shorten,
  lengthen,
  idExists,
  getStats,
};
