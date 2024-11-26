import { client } from '$lib/api/api';
import type { components } from '$lib/api/schema';

export type EditSystemNameRequest = components['schemas']['EditSystemNameRequest'];

export async function PATCH({ request, fetch }) {
	const { id, name } = (await request.json()) as EditSystemNameRequest;

	const res = await client.PATCH('/edit_system_name', {
		body: { id, name },
		fetch
	});

	return new Response('', { status: res.response.status });
}
