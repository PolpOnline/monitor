import { writable, type Writable } from 'svelte/store';

export const addSystemSheetOpen = writable(false);

export const deleteSystemDialogOpen = writable(false);
export const deleteSystemDialogId: Writable<string | null> = writable(null);

export const editSystemNameDialogOpen = writable(false);
export const editSystemNameDialogId: Writable<string | null> = writable(null);
export const editSystemNameDialogOldName: Writable<string | null> = writable(null);
