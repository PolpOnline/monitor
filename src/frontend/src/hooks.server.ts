import { API_URL } from '$lib/api/api';
import type { LoginStatusResponse } from '../../backend/bindings/index';
import type { HandleFetch, Handle } from '@sveltejs/kit';

// Forwards all cookies to the API, see https://kit.svelte.dev/docs/hooks#server-hooks-handlefetch
export const handleFetch: HandleFetch = async ({ event, request, fetch }) => {
	if (request.url.startsWith(API_URL)) {
		const cookies = event.request.headers.get('cookie');
		if (cookies) {
			request.headers.set('cookie', cookies);
		}
	}
	return fetch(request);
};

export const handle: Handle = async ({ event, resolve }) => {
	const requestedPath = event.url.pathname;

	if (requestedPath === '/login') {
		return await resolve(event);
	}

	// Auth check
	const status = await getLoginStatus(event.fetch);

	// Set the login status in the locals object, so we can access it in the page component
	event.locals.loginStatus = status.status;

	if (status.status === 'logged_out') {
		return new Response(null, {
			status: 302,
			headers: { location: '/login' }
		});
	}

	return await resolve(event);
};

async function getLoginStatus(fetch: WindowOrWorkerGlobalScope['fetch']) {
	return fetch(`${API_URL}/login_status`, {
		method: 'GET',
		headers: {
			'Content-Type': 'application/json'
		},
		credentials: 'include'
	}).then((res) => res.json() as Promise<LoginStatusResponse>);
}
