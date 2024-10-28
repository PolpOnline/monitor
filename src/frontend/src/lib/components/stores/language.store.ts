import { writable } from 'svelte/store';
import { browser } from '$app/environment';

export const language = writable<string>('en');

if (browser) {
	language.set(navigator.language.split('-')[0] || 'en');
}
