import { client } from '$lib/api/api';
import type { components } from '$lib/api/schema';

export type DeleteSystemRequest = components['schemas']['DeleteSystemRequest'];

export async function DELETE({ request, fetch }) {
	const { id } = (await request.json()) as DeleteSystemRequest;

	const res = await client.DELETE('/delete_system', { body: { id }, fetch });

	return new Response('', { status: res.response.status });
}
