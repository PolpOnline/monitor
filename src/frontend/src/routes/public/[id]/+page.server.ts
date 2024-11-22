import { API_URL, LIST_SIZE } from '$lib/api/api';
import type { PageServerLoad } from './$types';
import type { GetPublicResponse } from '$lib/bindings';

export const load: PageServerLoad = async ({ fetch, params, url }) => {
	const page = Number(url.searchParams.get('page')) || 0;

	const res = await fetch(
		`${API_URL}/get_public/${params.id}?list_size=${LIST_SIZE}&page=${page}`,
		{
			method: 'GET',
			headers: {
				'Content-Type': 'application/json'
			}
		}
	);

	if (!res.ok) {
		return new Response('Failed to fetch', { status: res.status });
	}

	const data = (await res.json()) as GetPublicResponse;

	return {
		system: data.system
	};
};
