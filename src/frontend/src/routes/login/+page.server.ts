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

		// If the request was not successful, return the status code and the form
		if (!res.ok) {
			return fail(res.status, {
				form
			});
		}

		const backendSetCookieUnparsed = res.headers.get('set-cookie');

		if (!backendSetCookieUnparsed) {
			return fail(500, {
				form
			});
		}

		const backendSetCookie = cookie.parse(backendSetCookieUnparsed);

		event.cookies.set('id', backendSetCookie.id, {
			sameSite: 'strict',
			path: '/',
			maxAge: parseInt(backendSetCookie['Max-Age']),
			httpOnly: true
		});

		redirect(303, '/');
	}
} satisfies Actions;
