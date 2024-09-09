import { writable, type Writable } from 'svelte/store';

export const addSystemSheetOpen = writable(false);

export const deleteSystemDialogOpen = writable(false);
export const deleteSystemDialogId: Writable<string | null> = writable(null);
