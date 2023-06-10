import { z } from "zod";

export const serverScheme = z.object({
  NODE_ENV: z
    .enum(["development", "production", "test"])
    .default("development"),
  ENABLE_VC_BUILD: z
    .string()
    .default("1")
    .transform((v) => parseInt(v)),
});

export const clientScheme = z.object({
  MODE: z.enum(["development", "production", "test"]).default("development"),
  VITE_API_URL: z.string().url(),
});
