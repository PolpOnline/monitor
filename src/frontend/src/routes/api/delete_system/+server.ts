import { API_URL } from '$lib/api/api';

export async function DELETE({ request, fetch }) {
	const { id } = await request.json();

	const res = await fetch(`${API_URL}/delete_system`, {
		method: 'DELETE',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify({ id })
	});

	return new Response(null, { status: res.status });
}
