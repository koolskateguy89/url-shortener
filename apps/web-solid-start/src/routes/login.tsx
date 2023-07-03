import type { VoidComponent } from "solid-js";
import { createRouteAction } from "solid-start";
import { z } from "zod";

import { type LoginRequest, api } from "api";
import { Button, Input, LoadingSpinner } from "ui";
import { WhoAmI } from "~/components/who-am-i";

const formDataSchema = z.object({
  username: z.string(),
  password: z.string().min(4),
}) satisfies z.ZodType<LoginRequest>;

const LoginPage: VoidComponent = () => {
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
    }
  );

  const isLoading = loggingIn.pending;

  const handleLogout = () => {
    void api.logout();
  };

  return (
    <main class="flex h-screen flex-col items-center justify-center space-y-4">
      <div class="mb-12 flex flex-col gap-y-4">
        <WhoAmI />

        <Button onClick={handleLogout} variant="destructive">
          LOG out
        </Button>
      </div>

      <Form class="flex flex-col items-center space-y-2">
        <Input
          name="username"
          placeholder="Username"
          autocomplete="username"
          disabled={isLoading}
        />

        <Input
          type="password"
          name="password"
          placeholder="Password"
          autocomplete="current-password"
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
