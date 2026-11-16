import { writable } from 'svelte/store';
import { dev } from '$app/env';

function createTitle() {
	const { subscribe, set } = writable('');

	return {
		subscribe,
		set: (value: string) => {
			set(`${dev ? '(DEV) ' : ''}${value}`);
		}
	};
}

export const title = createTitle();
