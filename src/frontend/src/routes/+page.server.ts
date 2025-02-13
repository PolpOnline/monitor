import { client } from '$lib/api/api.server';
import { LIST_SIZE } from '$lib/api/public-api';
import { formSchema } from '$lib/components/add_system/schema';
import { getReasonPhrase, StatusCodes } from 'http-status-codes';
import type { Actions, PageServerLoad } from './$types.js';
import { error, fail, redirect } from '@sveltejs/kit';
import { message, superValidate } from 'sveltekit-superforms';
import { zod } from 'sveltekit-superforms/adapters';

export const load: PageServerLoad = async ({ fetch, url }) => {
	const page = Number(url.searchParams.get('page')) || 0;

	const {
		data,
		error: errorMessage,
		response
	} = await client.GET('/list_systems', {
		params: { query: { list_size: LIST_SIZE, page } },
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
		systems: data!.systems,
		form: await superValidate(zod(formSchema))
	};
};

// noinspection JSUnusedGlobalSymbols
export const actions: Actions = {
	add_system: async (event) => {
		const form = await superValidate(event, zod(formSchema));

		if (!form.valid) {
			return fail(StatusCodes.BAD_REQUEST, {
				form
			});
		}

		const { response } = await client.POST('/add_system', { body: form.data, fetch: event.fetch });

		if (!response.ok) {
			return message(form, '', {
				// @ts-expect-error - assume res has a valid status code
				status: response.status
			});
		}

		return {
			form
		};
	}
};
