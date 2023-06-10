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
  const res = await api.lengthen(params.id);

  if ("error" in res) {
    const error = typeof res.error === "string" ? res.error : res.error.Other;

    redirect(`/${params.id}/error?cause=${error}`);
  }

  return <>{children}</>;
}
