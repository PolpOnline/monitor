import { API_URL } from '$lib/api/api';
import type { PageServerLoad } from './$types';
import { redirect } from '@sveltejs/kit';

export const load: PageServerLoad = async ({ fetch }) => {
	await fetch(`${API_URL}/logout`, {
		method: 'GET',
		headers: {
			'Content-Type': 'application/json'
		}
	});

	redirect(302, '/login');
};
