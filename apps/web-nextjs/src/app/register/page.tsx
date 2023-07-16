"use client";

import Link from "next/link";
import { useRouter } from "next/navigation";
import { useMutation } from "@tanstack/react-query";
import { z } from "zod";

import { type RegisterRequest, api } from "api";
import { Button, Input, LoadingSpinner } from "ui";

// TODO: make sure conforms to RegisterRequest with satisfies
const formDataSchema = z.object({
  username: z.string(),
  password: z.string().min(4),
}) satisfies z.ZodType<Partial<RegisterRequest>>;
// }) satisfies z.ZodType<RegisterRequest>;

export default function RegisterPage() {
  const router = useRouter();

  const registerMutation = useMutation({
    mutationFn: async (credentials: RegisterRequest) =>
      await api.register(credentials),
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
    const credentials = sfp.data as RegisterRequest;

    const registered = await api.register(credentials);
    alert(registered ? "Registered" : "Failed to register");

    if (registered) router.push("/login");
  };

  const isLoading = registerMutation.isLoading;

  return (
    <main className="flex h-screen flex-col items-center justify-center">
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
          autoComplete="new-password"
          disabled={isLoading}
          required
        />

        <div className="flex w-full justify-evenly">
          <Button variant="link" asChild>
            <Link href="/login">Login</Link>
          </Button>
          <Button type="submit" disabled={isLoading}>
            {isLoading && <LoadingSpinner className="mr-2" />}
            Register
          </Button>
        </div>
      </form>
    </main>
  );
}
