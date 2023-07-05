import { redirect } from "next/navigation";

import { api, errorUrl } from "api";

export default async function RedirectPage({
  params,
}: {
  params: {
    id: string;
  };
}) {
  const res = await api.lengthen(params.id);

  if (res.success) {
    redirect(res.data.url);
  } else {
    redirect(errorUrl(params.id, "NotFound"));
  }
}
