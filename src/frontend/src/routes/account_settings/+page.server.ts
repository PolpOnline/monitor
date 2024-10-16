import type { Actions, PageServerLoad } from './$types';
import { fail, redirect } from '@sveltejs/kit';
import { message, superValidate } from 'sveltekit-superforms';
import { formSchema as changePasswordFormSchema } from './change_password/schema';
import { formSchema as changeTimezoneFormSchema } from './change_timezone/schema';
import { zod } from 'sveltekit-superforms/adapters';
import { API_URL } from '$lib/api/api';
import type { GetCurrentSettingsResponse } from '$lib/bindings';
import { timezones } from '$lib/server/timezones/timezones';

export const load: PageServerLoad = async ({ fetch }) => {
	const currentSettings = await fetch(`${API_URL}/user/get_current_settings`, {
		method: 'GET',
		headers: {
			'Content-Type': 'application/json'
		}
	}).then(async (res) => (await res.json()) as Promise<GetCurrentSettingsResponse>);

	return {
		passwordForm: await superValidate(zod(changePasswordFormSchema)),
		timezoneForm: await superValidate(zod(changeTimezoneFormSchema), {
			defaults: {
				timezone: currentSettings.timezone
			}
		}),
		timezones
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

		event.cookies.set('id', '', {
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
	}
} satisfies Actions;
