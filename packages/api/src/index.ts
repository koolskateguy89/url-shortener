// TODO: env
const API_URL = "http://127.0.0.1:8000/api";

export interface ShortenResponse {
  id: string;
}

export interface LengthenResponse {
  url: string;
}

export interface ErrorResponse {
  error:
    | "NotFound"
    | "InvalidUrl"
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

async function lengthen(id: string): Promise<LengthenResponse | ErrorResponse> {
  const res = await fetch(`${API_URL}/${id}`);

  return (await res.json()) as LengthenResponse | ErrorResponse;
}

export const api = {
  shorten,
  lengthen,
};
