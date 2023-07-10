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

export interface LoginRequest {
  username: string;
  password: string;
}

export interface RegisterRequest {
  username: string;
  password: string;
}

export type Error = "NotFound" | "InvalidUrl" | (string & {});

export type AuthError =
  | "UserNotFound"
  | "UserIncorrectPassword"
  | "UsernameTaken"
  | "InvalidCredentials";

export interface ErrorResponse {
  error: Error;
}

type ApiResponse<T, F> =
  | {
      success: true;
      data: T;
    }
  | {
      success: false;
      error: F;
    };

function apiResponse<T extends object>(
  data: T | ErrorResponse
): ApiResponse<T, ErrorResponse> {
  if ("error" in data)
    return {
      success: false,
      error: data,
    };

  return {
    success: true,
    data,
  };
}

export function errorUrl(id: string, cause: Error): string {
  return `/error?id=${encodeURIComponent(id)}&cause=${encodeURIComponent(
    cause
  )}`;
}

/**
 * Error response should not occur, but if it does, it will be a 500.
 *
 * @param url
 * @returns
 */
async function shorten(
  url: string
): Promise<ApiResponse<ShortenResponse, ErrorResponse>> {
  const res = await fetch(`${API_URL}/url/shorten`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ url }),
  });

  const body = (await res.json()) as ShortenResponse | ErrorResponse;
  return apiResponse(body);
}

async function lengthen(
  id: string
): Promise<ApiResponse<LengthenResponse, ErrorResponse>> {
  const res = await fetch(`${API_URL}/url/${encodeURIComponent(id)}/lengthen`);

  const body = (await res.json()) as LengthenResponse | ErrorResponse;
  return apiResponse(body);
}

async function idExists(id: string): Promise<boolean> {
  const res = await fetch(`${API_URL}/url/${id}/exists`);

  return res.ok;
}

async function getStats(
  id: string,
  init: RequestInit = {
    cache: "no-cache",
  }
): Promise<ApiResponse<StatsResponse, ErrorResponse>> {
  const res = await fetch(
    `${API_URL}/url/${encodeURIComponent(id)}/stats`,
    init
  );

  const body = (await res.json()) as StatsResponse | ErrorResponse;
  return apiResponse(body);
}

async function whoami(): Promise<string> {
  const res = await fetch(`${API_URL}/whoami`);

  return res.text();
}

async function login(credentials: LoginRequest): Promise<boolean> {
  const res = await fetch(`${API_URL}/login`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(credentials),
  });

  return res.ok;
}

async function logout(): Promise<boolean> {
  const res = await fetch(`${API_URL}/logout`, {
    method: "POST",
  });

  return res.ok;
}

async function register(body: RegisterRequest): Promise<boolean> {
  const res = await fetch(`${API_URL}/register`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(body),
  });

  return res.ok;
}

export const api = {
  shorten,
  lengthen,
  idExists,
  getStats,
  whoami,
  login,
  logout,
  register,
};
