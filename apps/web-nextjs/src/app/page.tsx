"use client";

import { useMutation } from "@tanstack/react-query";
import { z } from "zod";

import { api } from "api";
import { Button, Input, LoadingSpinner } from "ui";
import { StatusDisplay } from "./status-display";

const formDataSchema = z.object({
  url: z.string().url(),
});

export default function HomePage() {
  // FIXME?: isn't erroring when the request fails
  const shortenMutation = useMutation({
    mutationFn: async (url: string) => await api.shorten(url),
  });

  const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();

    const formData = new FormData(e.currentTarget);

    const wrappedUrl = formDataSchema.safeParse(Object.fromEntries(formData));

    if (!wrappedUrl.success) {
      alert("Invalid URL");
      return;
    }

    const { url } = wrappedUrl.data;
    shortenMutation.mutate(url);
  };

  const isLoading = shortenMutation.isLoading;

  return (
    <main className="flex h-screen flex-col items-center justify-center space-y-4">
      <StatusDisplay {...shortenMutation} />

      <form
        onSubmit={(e) => void handleSubmit(e)}
        className="flex flex-col items-center space-y-2"
      >
        <Input type="url" name="url" placeholder="Url" disabled={isLoading} />

        <Button type="submit" disabled={isLoading}>
          {isLoading && <LoadingSpinner className="mr-2" />}
          Shorten
        </Button>
      </form>
    </main>
  );
}
