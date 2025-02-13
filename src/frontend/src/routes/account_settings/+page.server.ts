import type { Actions, PageServerLoad } from './$types';
import { fail, redirect } from '@sveltejs/kit';
import { message, superValidate } from 'sveltekit-superforms';
import { formSchema as changePasswordFormSchema } from './change_password/schema';
import { formSchema as changeTimezoneFormSchema } from './change_timezone/schema';
import { formSchema as changeLanguageFormSchema } from './change_language/schema';
import { zod } from 'sveltekit-superforms/adapters';
import { API_URL, client } from '$lib/api/api.server';

export const load: PageServerLoad = async ({ fetch }) => {
	const {
		data: currentSettings,
		error,
		response
	} = await client.GET('/user/get_current_settings', {
		fetch
	});

	if (error) {
		return new Response(`Failed to fetch: ${error}`, { status: response.status });
	}

	return {
		passwordForm: await superValidate(zod(changePasswordFormSchema)),
		timezoneForm: await superValidate(zod(changeTimezoneFormSchema), {
			defaults: {
				timezone: currentSettings!.timezone
			}
		}),
		languageForm: await superValidate(zod(changeLanguageFormSchema), {
			defaults: {
				language: currentSettings!.language
			}
		})
	};
};

// noinspection JSUnusedGlobalSymbols
export const actions: Actions = {
	change_password: async (event) => {
		const form = await superValidate(event, zod(changePasswordFormSchema));

		// If the form is not valid, return a 400 error
		if (!form.valid) {
			return fail(400, {
				passwordForm: form
			});
		}

		const res = await event.fetch(`${API_URL}/user/change_password`, {
			method: 'PATCH',
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

		event.cookies.set('monitor_id', '', {
			sameSite: 'strict',
			path: '/',
			maxAge: 0,
			httpOnly: true
		});

		redirect(303, '/login');
	},

	change_timezone: async (event) => {
		const form = await superValidate(event, zod(changeTimezoneFormSchema));

		// If the form is not valid, return a 400 error
		if (!form.valid) {
			return fail(400, {
				timezoneForm: form
			});
		}

		const res = await event.fetch(`${API_URL}/user/change_timezone`, {
			method: 'PATCH',
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

		return { form };
	},

	change_language: async (event) => {
		const form = await superValidate(event, zod(changeLanguageFormSchema));

		// If the form is not valid, return a 400 error
		if (!form.valid) {
			return fail(400, {
				languageForm: form
			});
		}

		const res = await event.fetch(`${API_URL}/user/change_language`, {
			method: 'PATCH',
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

		return { form };
	}
} satisfies Actions;
