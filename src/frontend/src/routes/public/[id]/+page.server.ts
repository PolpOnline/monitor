import { client, LIST_SIZE } from '$lib/api/api';
import { getReasonPhrase, StatusCodes } from 'http-status-codes';
import type { PageServerLoad } from './$types';
import { error, redirect } from '@sveltejs/kit';

export const load: PageServerLoad = async ({ fetch, params, url }) => {
	const page = Number(url.searchParams.get('page')) || 0;

	const {
		data,
		error: errorMessage,
		response
	} = await client.GET('/get_public/{id}', {
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

	if (response.status === StatusCodes.UNAUTHORIZED) {
		redirect(StatusCodes.MOVED_TEMPORARILY, '/login');
	}

	if (errorMessage) {
		error(StatusCodes.INTERNAL_SERVER_ERROR, getReasonPhrase(StatusCodes.INTERNAL_SERVER_ERROR));
	}

	if (!data) {
		error(StatusCodes.INTERNAL_SERVER_ERROR, getReasonPhrase(StatusCodes.INTERNAL_SERVER_ERROR));
	}

	return {
		system: data.system
	};
};
