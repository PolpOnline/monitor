import 'unplugin-icons/types/svelte';
export type LoginStatus = 'logged_in' | 'logged_out';

// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
declare global {
	namespace App {
		// interface Error {}
		// noinspection JSUnusedGlobalSymbols
		interface Locals {
			loginStatus: LoginStatus;
			email?: string;
		}
		// interface PageData {}
		// interface PageState {}
		// interface Platform {}
	}
}

export {};
