import { client } from '$lib/api/api';
import type { PageServerLoad } from './$types';
import { redirect } from '@sveltejs/kit';

export const load: PageServerLoad = async ({ fetch }) => {
	await client.GET('/logout', { fetch });

	redirect(302, '/login');
};
