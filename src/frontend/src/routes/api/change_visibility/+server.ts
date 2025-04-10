import { client } from '$lib/api/api.server';
import type { components } from '$lib/api/schema';

export type ChangeVisibilityRequest = components['schemas']['ChangeVisibilityRequest'];

export async function PATCH({ request, fetch }) {
	const { id, visibility } = (await request.json()) as ChangeVisibilityRequest;

	const res = await client.PATCH('/change_visibility', {
		body: { id, visibility },
		fetch
	});

	return new Response('', { status: res.response.status });
}
