import { redirect } from "next/navigation";

import { type Error, api } from "api";

export default async function Layout({
  children,
  params,
}: {
  children: React.ReactNode;
  params: {
    id: string;
  };
}) {
  const exists = await api.idExists(params.id);

  if (!exists) {
    redirect(
      `/error?id=${encodeURIComponent(params.id)}&cause=${
        "NotFound" satisfies Error
      }`
    );
  }

  return <>{children}</>;
}
