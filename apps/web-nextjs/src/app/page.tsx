"use client";

import { useState } from "react";
import Link from "next/link";
import { z } from "zod";

import { api } from "api";
import { Button, Input, LoadingSpinner } from "ui";

const formDataSchema = z.object({
  url: z.string().url(),
});

// TODO?: use react query or smthn for handling server state

export default function Page() {
  const [isLoading, setIsLoading] = useState(false);
  const [id, setId] = useState<string | null>(null);

  const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();

    const formData = new FormData(e.currentTarget);

    const wrappedUrl = formDataSchema.safeParse(Object.fromEntries(formData));

    if (!wrappedUrl.success) {
      alert("Invalid URL");
      return;
    }

    const { url } = wrappedUrl.data;

    setIsLoading(true);

    const res = await api.shorten(url);
    setId(res.id);
    setIsLoading(false);
  };

  return (
    <main className="space-y-4' flex h-screen flex-col items-center justify-center">
      <p>
        TODO: display status (properly)
        {id && (
          <>
            <br />
            <Link href={`/${id}`} className="underline">
              BASE_URL/{id}
            </Link>
          </>
        )}
      </p>

      <form
        onSubmit={(e) => void handleSubmit(e)}
        className="flex flex-col items-center space-y-2"
      >
        <Input type="url" name="url" placeholder="Url" disabled={isLoading} />

        <Button type="submit" disabled={isLoading}>
          <LoadingSpinner className="mr-2" />
          Shorten
        </Button>
      </form>
    </main>
  );
}
