import { writable, type Writable } from 'svelte/store';
import type { components } from '$lib/api/schema';

export const addSystemSheetOpen = writable(false);

export const deleteSystemDialogOpen = writable(false);

export const editSystemNameDialogOpen = writable(false);

export const presetDialogOpen = writable(false);

export const targetSystemData: Writable<components['schemas']['SystemData'] | undefined> =
	writable(undefined);
