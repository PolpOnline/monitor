import type {
	// Instant,
	// Status,
	SystemData,
	ListSystemsResponse
} from '../../../backend/bindings/index';
import { API_URL } from '$lib/api/api';
import { formSchema } from '$lib/components/add_system/schema';
import type { PageServerLoad, Actions } from './$types.js';
import { fail } from '@sveltejs/kit';
import { superValidate, message } from 'sveltekit-superforms';
import { zod } from 'sveltekit-superforms/adapters';

// function generateRandomStatus(): Status {
// 	const statuses: Status[] = ['up', 'down'];
// 	return statuses[Math.floor(Math.random() * statuses.length)];
// }
//
// function generateRandomSystemData(name: string, frequency: number): SystemData {
// 	const instants: Instant[] = [];
// 	const startTime = new Date(Date.now());
//
// 	for (let i = 0; i < 30; i++) {
// 		const timestamp = new Date(startTime.getTime() - i * frequency * 60 * 1000);
//
// 		instants.push({
// 			status: generateRandomStatus(),
// 			timestamp,
// 			expected_timestamp: timestamp
// 		});
// 	}
//
// 	instants.reverse();
//
// 	return {
// 		name,
// 		instants,
// 		frequency
// 	};
// }

// const exampleSystemData1 = generateRandomSystemData('Test System 1', 30); // 30 minutes
// const exampleSystemData2 = generateRandomSystemData('Test System 2', 60); // 1 hour
// const exampleSystemData3 = generateRandomSystemData('Test System 3', 180); // 3 hours

export const load: PageServerLoad = async ({ fetch }) => {
	const response = await getSystemsList(fetch);

	return {
		systems: response.systems as SystemData[],
		form: await superValidate(zod(formSchema))
	};
};

async function getSystemsList(
	fetch: WindowOrWorkerGlobalScope['fetch']
): Promise<ListSystemsResponse> {
	return fetch(`${API_URL}/list_systems`, {
		method: 'GET',
		headers: {
			'Content-Type': 'application/json'
		}
	}).then((res) => res.json() as Promise<ListSystemsResponse>);
}

// noinspection JSUnusedGlobalSymbols
export const actions: Actions = {
	add_system: async (event) => {
		const form = await superValidate(event, zod(formSchema));

		if (!form.valid) {
			return fail(400, {
				form
			});
		}

		const response = await event.fetch(`${API_URL}/add_system`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(form.data)
		});

		const messageToSend = await response.text();

		if (!response.ok) {
			return message(form, messageToSend, {
				// @ts-expect-error - assume res has a valid status code
				status: response.status
			});
		}

		return {
			form
		};
	}
};
