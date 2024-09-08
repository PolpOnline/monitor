import type { Actions, PageServerLoad } from './$types';
import { fail, redirect } from '@sveltejs/kit';
import { superValidate } from 'sveltekit-superforms';
import { formSchema } from './schema';
import { zod } from 'sveltekit-superforms/adapters';
import { API_URL } from '$lib/api/api';
import cookie from 'cookie';

export const load: PageServerLoad = async () => {
	return {
		form: await superValidate(zod(formSchema))
	};
};

// noinspection JSUnusedGlobalSymbols
export const actions: Actions = {
	change_password: async (event) => {
		const form = await superValidate(event, zod(formSchema));

		// If the form is not valid, return a 400 error
		if (!form.valid) {
			return fail(400, {
				form
			});
		}

		const res = await event.fetch(`${API_URL}/user/change_password`, {
			method: 'PATCH',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(form.data)
		});

		console.log(res);

		// If the request was not successful, return the status code and the form
		if (!res.ok) {
			return fail(res.status, {
				form
			});
		}

		event.cookies.set('id', '', {
			sameSite: 'strict',
			path: '/',
			maxAge: 0,
			httpOnly: true
		});

		redirect(303, '/login');
	}
} satisfies Actions;
