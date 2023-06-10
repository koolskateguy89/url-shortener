import { LoadingSpinner } from "ui";

export default function Loading() {
  return (
    <main className="flex h-screen items-center justify-center">
      <LoadingSpinner className="h-8 w-8" />
    </main>
  );
}
