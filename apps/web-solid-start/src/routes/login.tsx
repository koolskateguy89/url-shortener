import { type VoidComponent, createResource } from "solid-js";
import { createRouteAction } from "solid-start";
import { z } from "zod";

import { type LoginRequest, api } from "api";
import { Button, Input, LoadingSpinner } from "ui";

const formDataSchema = z.object({
  username: z.string(),
  password: z.string().min(4),
}) satisfies z.ZodType<LoginRequest>;

const LoginPage: VoidComponent = () => {
  const [whoami, { refetch }] = createResource(async () => {
    return await api.whoami();
  });

  const [loggingIn, { Form }] = createRouteAction(
    async (formData: FormData) => {
      const sfp = formDataSchema.safeParse(Object.fromEntries(formData));

      if (!sfp.success) {
        alert("Invalid credentials");
        return;
      }

      const credentials = sfp.data;

      const loggedIn = await api.login(credentials);
      alert(loggedIn ? "Logged in" : "Failed to log in");

      await refetch();
    }
  );

  const isLoading = loggingIn.pending;

  const handleLogout = async () => {
    await api.logout();
    await refetch();
  };

  return (
    <main class="flex h-screen flex-col items-center justify-center space-y-4">
      <div class="mb-20 flex flex-col gap-y-4">
        <pre>
          me ={" "}
          <code>
            {whoami.loading && <LoadingSpinner class="mr-2 inline" />}
            {JSON.stringify(whoami(), null, 2)}
          </code>
        </pre>
        <Button onClick={() => void handleLogout()} variant="destructive">
          LOG out
        </Button>
      </div>

      <Form class="flex flex-col items-center space-y-2">
        <Input
          name="username"
          placeholder="Username"
          auto-complete="username"
          disabled={isLoading}
        />

        <Input
          type="password"
          name="password"
          placeholder="Password"
          auto-complete="current-password"
          disabled={isLoading}
        />

        <Button type="submit" disabled={isLoading}>
          {isLoading && <LoadingSpinner class="mr-2" />}
          Login
        </Button>
      </Form>
    </main>
  );
};

export default LoginPage;
