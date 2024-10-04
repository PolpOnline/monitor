import { API_URL } from '$lib/api/api';
import type { Handle, HandleFetch } from '@sveltejs/kit';
import type { LoginStatus } from './app';
import { default as setCookieParser } from 'set-cookie-parser';

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

	if (!setCookieHeader) {
		return res;
	}

	const setCookies = setCookieParser.parse(setCookieHeader);

	// Response did not contain a set-cookie header
	if (!setCookies) {
		return res;
	}

	// Forward the set-cookie header to the client
	setCookies.forEach((cookie) => {
		event.cookies.set(cookie.name, cookie.value, {
			sameSite: 'strict',
			path: '/',
			maxAge: cookie.maxAge || 99999999,
			httpOnly: true,
			secure: true
		});
	});

	return res;
};

export const handle: Handle = async ({ event, resolve }) => {
	const requestedPath = event.url.pathname;
	// Auth check
	event.locals.loginStatus = event.cookies.get('id') ? 'logged_in' : ('logged_out' as LoginStatus);

	if (requestedPath === '/login' || requestedPath.startsWith('/public/')) {
		return await resolve(event);
	}

	if (event.locals.loginStatus === 'logged_out') {
		return new Response(null, {
			status: 302,
			headers: { location: '/login' }
		});
	}

	// Set the login status in the "locals" object, so we can access it in the page component
	event.locals.email = event.cookies.get('user_email');

	return await resolve(event);
};
