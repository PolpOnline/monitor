import { client, LIST_SIZE } from '$lib/api/api';
import { formSchema } from '$lib/components/add_system/schema';
import type { Actions, PageServerLoad } from './$types.js';
import { fail } from '@sveltejs/kit';
import { message, superValidate } from 'sveltekit-superforms';
import { zod } from 'sveltekit-superforms/adapters';

export const load: PageServerLoad = async ({ fetch, url }) => {
	const page = Number(url.searchParams.get('page')) || 0;

	const { data, error, response } = await client.GET('/list_systems', {
		params: { query: { list_size: LIST_SIZE, page } },
		fetch
	});

	if (error || !data || !data.systems) {
		return new Response(`Failed to fetch: ${error}`, { status: response.status });
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
			return fail(400, {
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
