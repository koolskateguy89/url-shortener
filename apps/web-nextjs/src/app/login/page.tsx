"use client";

import Link from "next/link";
import { useMutation } from "@tanstack/react-query";
import { z } from "zod";

import { type LoginRequest, api } from "api";
import { Button, Input, LoadingSpinner } from "ui";
import { WhoAmI } from "./who-am-i";

// TODO: make sure conforms to LoginRequest with satisfies
// but for some reason it thinks it outputs a Partial<_>
// it works fine in the solid version
// idrk what to do about this, I have no idea why it isn't working
// I've tried reinstalling `node_modules` and it's still the same
const formDataSchema = z.object({
  username: z.string(),
  password: z.string().min(4),
}) satisfies z.ZodType<Partial<LoginRequest>>;
// }) satisfies z.ZodType<LoginRequest>;

export default function LoginPage() {
  const loginMutation = useMutation({
    mutationFn: async (credentials: LoginRequest) =>
      await api.login(credentials),
  });

  const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();

    const formData = new FormData(e.currentTarget);

    const sfp = formDataSchema.safeParse(Object.fromEntries(formData));

    if (!sfp.success) {
      alert("Invalid credentials");
      return;
    }

    // TODO: remove type cast once above TODO is fixed
    const credentials = sfp.data as LoginRequest;

    const loggedIn = await loginMutation.mutateAsync(credentials);
    alert(loggedIn ? "Logged in" : "Failed to log in");
  };

  const handleLogout = async () => {
    try {
      const loggedOut = await api.logout();
      alert(loggedOut ? "Logged out" : "Failed to log out");
    } catch (err) {
      console.error(err);
      alert("Failed to log out (errored, check console)");
    }
  };

  const isLoading = loginMutation.isLoading;

  return (
    <main className="flex h-screen flex-col items-center justify-center">
      <div className="mb-12 flex flex-col gap-y-4">
        <WhoAmI />

        <Button onClick={() => void handleLogout()} variant="destructive">
          LOG out
        </Button>
      </div>

      <form
        onSubmit={(e) => void handleSubmit(e)}
        className="flex flex-col items-center gap-y-2"
      >
        <Input
          name="username"
          placeholder="Username"
          autoComplete="username"
          disabled={isLoading}
          required
        />

        <Input
          type="password"
          name="password"
          placeholder="Password"
          autoComplete="current-password"
          disabled={isLoading}
          required
        />

        <div>
          <Button type="submit" disabled={isLoading}>
            {isLoading && <LoadingSpinner className="mr-2" />}
            Login
          </Button>
          <Button variant="link" asChild>
            <Link href="/register">Register</Link>
          </Button>
        </div>
      </form>
    </main>
  );
}
