import { API_URL, LIST_SIZE } from '$lib/api/api';
import type { PageServerLoad } from './$types';
import type { GetPublicResponse } from '../../../../../backend/bindings';

export const load: PageServerLoad = async ({ fetch, params }) => {
	const res = await fetch(`${API_URL}/get_public/${params.id}?list_size=${LIST_SIZE}`, {
		method: 'GET',
		headers: {
			'Content-Type': 'application/json'
		}
	}).then((res) => res.json() as Promise<GetPublicResponse>);

	return {
		system: res.system
	};
};
