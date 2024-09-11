import type { Actions, PageServerLoad } from './$types';
import { fail, redirect } from '@sveltejs/kit';
import { superValidate, message } from 'sveltekit-superforms';
import { formSchema } from './schema';
import { zod } from 'sveltekit-superforms/adapters';
import { API_URL } from '$lib/api/api';

export const load: PageServerLoad = async () => {
	return {
		form: await superValidate(zod(formSchema))
	};
};

export const actions: Actions = {
	default: async (event) => {
		const form = await superValidate(event, zod(formSchema));

		// If the form is not valid, return a 400 error
		if (!form.valid) {
			return fail(400, {
				form
			});
		}

		const res = await event.fetch(`${API_URL}/login`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(form.data)
		});

		const messageToSend = await res.text();

		// If the request was not successful, return the status code and the form
		if (!res.ok) {
			return message(form, messageToSend, {
				// @ts-expect-error - assume res has a valid status code
				status: res.status
			});
		}

		// Cookie is handled by handleFetch in hooks.server.ts

		redirect(303, '/');
	}
} satisfies Actions;
