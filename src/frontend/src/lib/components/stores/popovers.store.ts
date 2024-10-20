import { writable, type Writable } from 'svelte/store';
import type { SystemData } from '$lib/bindings';

export const addSystemSheetOpen = writable(false);

export const deleteSystemDialogOpen = writable(false);

export const editSystemNameDialogOpen = writable(false);

export const presetDialogOpen = writable(false);

export const targetSystemData: Writable<SystemData | undefined> = writable(undefined);
