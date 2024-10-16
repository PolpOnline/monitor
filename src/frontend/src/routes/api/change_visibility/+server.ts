import { API_URL } from '$lib/api/api';
import type { ChangeVisibilityRequest } from '$lib/bindings';

export async function PATCH({ request, fetch }) {
	const { id, visibility } = (await request.json()) as ChangeVisibilityRequest;

	const res = await fetch(`${API_URL}/change_visibility`, {
		method: 'PATCH',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify({ id, visibility } as ChangeVisibilityRequest)
	});

	return new Response(await res.text(), { status: res.status });
}
