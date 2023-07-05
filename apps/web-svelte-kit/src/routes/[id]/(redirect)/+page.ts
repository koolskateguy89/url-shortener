import { redirect } from '@sveltejs/kit';
import type { PageLoad } from './$types';

import { api, errorUrl } from 'api';

export const load = (async ({ params }) => {
	const res = await api.lengthen(params.id);

	if (res.success) {
		throw redirect(302, res.data.url);
	} else {
		throw redirect(307, errorUrl(params.id, 'NotFound'));
	}
}) satisfies PageLoad;
