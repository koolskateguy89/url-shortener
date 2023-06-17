import { redirect } from "next/navigation";

import { api } from "api";

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
    redirect(`/${params.id}/error?cause=404`);
  }

  return <>{children}</>;
}
