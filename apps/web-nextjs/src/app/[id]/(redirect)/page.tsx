import { redirect } from "next/navigation";

import { type LengthenResponse, api } from "api";

export default async function RedirectPage({
  params,
}: {
  params: {
    id: string;
  };
}) {
  // ID is checked in layout file
  const { url } = (await api.lengthen(params.id)) as LengthenResponse;

  redirect(url);
}
