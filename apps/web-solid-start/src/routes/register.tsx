import type { VoidComponent } from "solid-js";
import { createRouteAction, redirect, Title } from "solid-start";
import { A } from "solid-start";
import { z } from "zod";

import { type RegisterRequest, api } from "api";
import { As, Button, Input, LoadingSpinner } from "ui";

const formDataSchema = z.object({
  username: z.string(),
  password: z.string().min(4),
}) satisfies z.ZodType<RegisterRequest>;

const RegisterPage: VoidComponent = () => {
  const [registering, { Form }] = createRouteAction(
    async (formData: FormData) => {
      const sfp = formDataSchema.safeParse(Object.fromEntries(formData));

      if (!sfp.success) {
        alert("Invalid credentials");
        return;
      }

      const credentials = sfp.data;

      const registered = await api.register(credentials);
      alert(registered ? "Registered" : "Failed to register");

      if (registered) throw redirect("/login");
    },
  );

  const isLoading = registering.pending;

  return (
    <main class="flex h-screen flex-col items-center justify-center">
      <Title>Register</Title>

      <Form class="flex flex-col items-center gap-y-2">
        <Input
          name="username"
          placeholder="Username"
          autocomplete="username"
          disabled={isLoading}
          required
        />

        <Input
          type="password"
          name="password"
          placeholder="Password"
          autocomplete="new-password"
          disabled={isLoading}
          required
        />

        <div class="flex w-full justify-evenly">
          <Button variant="link" asChild>
            <As component={A} href="/login">
              Login
            </As>
          </Button>
          <Button type="submit" disabled={isLoading}>
            {isLoading && <LoadingSpinner class="mr-2" />}
            Register
          </Button>
        </div>
      </Form>
    </main>
  );
};

export default RegisterPage;
