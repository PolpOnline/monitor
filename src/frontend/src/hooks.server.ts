import { API_URL } from '$lib/api/api';
import type { LoginStatusResponse } from '../../backend/bindings';
import type { Handle, HandleFetch } from '@sveltejs/kit';
import cookie from 'cookie';

// Forwards all cookies to the API, see https://kit.svelte.dev/docs/hooks#server-hooks-handlefetch
export const handleFetch: HandleFetch = async ({ event, request, fetch }) => {
	const isApiRequest = request.url.startsWith(API_URL);

	if (!isApiRequest) {
		return fetch(request);
	}

	// Forward all cookies to the API
	const cookies = event.request.headers.get('cookie');
	if (cookies) {
		request.headers.set('cookie', cookies);
	}

	const res = await fetch(request);

	// Check if the response contains a set-cookie header and set the cookie in to the client if it does
	const setCookieHeader = res.headers.get('set-cookie');

	// Response is a normal request, no need to do anything
	if (!setCookieHeader) {
		return res;
	}

	const backendSetCookie = cookie.parse(setCookieHeader);

	event.cookies.set('id', backendSetCookie.id, {
		sameSite: 'strict',
		path: '/',
		maxAge: parseInt(backendSetCookie['Max-Age']),
		httpOnly: true,
		secure: true
	});

	return res;
};

export const handle: Handle = async ({ event, resolve }) => {
	// Auth check
	const status = await getLoginStatus(event.fetch);

	// Set the login status in the "locals" object, so we can access it in the page component
	event.locals.loginStatus = status;

	const requestedPath = event.url.pathname;

	if (requestedPath === '/login' || requestedPath.startsWith('/public/')) {
		return await resolve(event);
	}

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
		}
	}).then((res) => res.json() as Promise<LoginStatusResponse>);
}
