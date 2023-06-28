"use client";

import { useMutation, useQuery } from "@tanstack/react-query";
import { z } from "zod";

import { type LoginRequest, api } from "api";
import { Button, Input, LoadingSpinner } from "ui";

// TODO: make sure conforms to LoginRequest with satisfies
// but for some reason it thinks it outputs a Partial<_>
// it works fine in the solid version
const formDataSchema = z.object({
  username: z.string(),
  password: z.string().min(4),
}) satisfies z.ZodType<Partial<LoginRequest>>;
// }) satisfies z.ZodType<LoginRequest>;

export default function LoginPage() {
  const whoAmIQuery = useQuery({
    queryKey: ["whoami"],
    queryFn: async () => {
      await new Promise((resolve) => setTimeout(resolve, 3000));

      const a = await api.whoami();
      console.log("WHO AM i:", a);

      return a;
    },
  });

  const loginMutation = useMutation({
    mutationFn: async (credentials: LoginRequest) =>
      await api.login(credentials),
    onSettled: () => {
      console.log("onSettled");
      void whoAmIQuery.refetch();
      console.log("onSettled (after)");
    },
  });

  const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();

    const formData = new FormData(e.currentTarget);

    const sfp = formDataSchema.safeParse(Object.fromEntries(formData));

    if (!sfp.success) {
      alert("Invalid credentials");
      return;
    }

    // TODO: remove as once above TODO fixed
    const credentials = sfp.data as LoginRequest;

    const loggedIn = await loginMutation.mutateAsync(credentials);
    alert(loggedIn ? "Logged in" : "Failed to log in");
  };

  const isLoading = loginMutation.isLoading;

  const handleLogout = () => api.logout().then(() => whoAmIQuery.refetch());

  return (
    <main className="flex h-screen flex-col items-center justify-center space-y-4">
      <div className="mb-20 flex flex-col gap-y-4">
        <pre>
          me ={" "}
          <code>
            {whoAmIQuery.isFetching && (
              <LoadingSpinner className="mr-2 inline" />
            )}
            {JSON.stringify(whoAmIQuery.data, null, 2)}
          </code>
        </pre>
        <Button onClick={() => void handleLogout()} variant="destructive">
          LOG out
        </Button>
      </div>

      <form
        onSubmit={(e) => void handleSubmit(e)}
        className="flex flex-col items-center space-y-2"
      >
        <Input
          name="username"
          placeholder="Username"
          autoComplete="username"
          disabled={isLoading}
        />

        <Input
          type="password"
          name="password"
          placeholder="Password"
          autoComplete="current-password"
          disabled={isLoading}
        />

        <Button type="submit" disabled={isLoading}>
          {isLoading && <LoadingSpinner className="mr-2" />}
          Login
        </Button>
      </form>
    </main>
  );
}
