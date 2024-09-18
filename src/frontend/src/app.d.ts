import 'unplugin-icons/types/svelte';
import type { LoginStatusResponse } from '../../backend/bindings';

// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
declare global {
	namespace App {
		// interface Error {}
		// noinspection JSUnusedGlobalSymbols
		interface Locals {
			loginStatus: LoginStatusResponse;
		}

		// interface PageData {}
		// interface PageState {}
		// interface Platform {}
	}
}

export {};
