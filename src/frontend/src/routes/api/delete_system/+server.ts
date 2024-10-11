import { API_URL } from '$lib/api/api';
import type { DeleteSystemRequest } from '../../../../../backend/bindings';

export async function DELETE({ request, fetch }) {
	const { id } = (await request.json()) as DeleteSystemRequest;

	const res = await fetch(`${API_URL}/delete_system`, {
		method: 'DELETE',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify({ id } as DeleteSystemRequest)
	});

	return new Response(await res.text(), { status: res.status });
}
