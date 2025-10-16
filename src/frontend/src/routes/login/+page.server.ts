import type { Actions, PageServerLoad } from './$types';
import { fail, redirect } from '@sveltejs/kit';
import { message, superValidate } from 'sveltekit-superforms';
import { formSchema } from './schema';
import { zod4 } from 'sveltekit-superforms/adapters';
import { client } from '$lib/api/api.server';
import { StatusCodes } from 'http-status-codes';

export const load: PageServerLoad = async () => {
	return {
		form: await superValidate(zod4(formSchema))
	};
};

export const actions: Actions = {
	default: async (event) => {
		const form = await superValidate(event, zod4(formSchema));

		// If the form is not valid, return a 400 error
		if (!form.valid) {
			return fail(StatusCodes.BAD_REQUEST, {
				form
			});
		}

		const res = await client.POST(`/login`, {
			content: form.data,
			fetch: event.fetch
		});

		// If the request was not successful, return the status code and the form
		if (!res.response.ok) {
			const messageToSend = res.response.statusText;

			return message(form, messageToSend, {
				// @ts-expect-error - assume res has a valid status code
				status: res.status
			});
		}

		// Cookie is handled by handleFetch in hooks.server.ts

		redirect(StatusCodes.SEE_OTHER, '/');
	}
} satisfies Actions;
