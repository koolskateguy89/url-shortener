import { redirect } from '@sveltejs/kit';
import type { PageLoad } from './$types';

import { api, errorUrl } from 'api';

export const load = (async ({ params }) => {
	const res = await api.getStats(params.id);

	if (res.success) {
		return {
			id: params.id,
			...res.data
		};
	} else {
		throw redirect(307, errorUrl(params.id, 'NotFound'));
	}
}) satisfies PageLoad;
