import type { VoidComponent } from "solid-js";
import { createRouteAction, A, Title } from "solid-start";
import { z } from "zod";

import { type LoginRequest, api } from "api";
import { As, Button, Input, LoadingSpinner } from "ui";
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

  const handleLogout = async () => {
    try {
      const loggedOut = await api.logout();
      alert(loggedOut ? "Logged out" : "Failed to log out");
    } catch (err) {
      console.error(err);
      alert("Failed to log out (errored, check console)");
    }
  };

  return (
    <main class="flex h-screen flex-col items-center justify-center">
      <Title>Login</Title>

      <div class="mb-12 flex flex-col gap-y-4">
        <WhoAmI />

        <Button onClick={() => void handleLogout()} variant="destructive">
          LOG out
        </Button>
      </div>

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
          autocomplete="current-password"
          disabled={isLoading}
          required
        />

        <div>
          <Button type="submit" disabled={isLoading}>
            {isLoading && <LoadingSpinner class="mr-2" />}
            Login
          </Button>
          <Button variant="link" asChild>
            <As component={A} href="/register">
              Register
            </As>
          </Button>
        </div>
      </Form>
    </main>
  );
};

export default LoginPage;
