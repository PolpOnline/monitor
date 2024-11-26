import { client, LIST_SIZE } from '$lib/api/api';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ fetch, params, url }) => {
	const page = Number(url.searchParams.get('page')) || 0;

	const { data, error, response } = await client.GET('/get_public/{id}', {
		params: {
			path: {
				id: params.id
			},
			query: {
				list_size: LIST_SIZE,
				page
			}
		},
		fetch
	});

	if (error || !data || !data.system) {
		return new Response(`Failed to fetch: ${error}`, { status: response.status });
	}

	return {
		system: data.system
	};
};
