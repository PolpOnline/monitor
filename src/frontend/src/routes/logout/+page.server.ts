import { client } from '$lib/api/api';
import { StatusCodes } from 'http-status-codes';
import type { PageServerLoad } from './$types';
import { redirect } from '@sveltejs/kit';

export const load: PageServerLoad = async ({ fetch }) => {
	await client.GET('/logout', { fetch });

	redirect(StatusCodes.MOVED_TEMPORARILY, '/login');
};
