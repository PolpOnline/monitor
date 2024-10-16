import { API_URL } from '$lib/api/api';
import type { EditSystemNameRequest } from '$lib/bindings';

export async function PATCH({ request, fetch }) {
	const { id, name } = (await request.json()) as EditSystemNameRequest;

	const res = await fetch(`${API_URL}/edit_system_name`, {
		method: 'PATCH',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify({ id, name } as EditSystemNameRequest)
	});

	return new Response(await res.text(), { status: res.status });
}
