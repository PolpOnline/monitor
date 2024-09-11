import { API_URL } from '$lib/api/api';
import type { EditSystemNameRequest } from '../../../../../backend/bindings';

export async function PATCH({ request, fetch }) {
	const { id, newSystemName } = await request.json();

	const res = await fetch(`${API_URL}/edit_system_name`, {
		method: 'PATCH',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify({ id, name: newSystemName } as EditSystemNameRequest)
	});

	return new Response(null, { status: res.status });
}
