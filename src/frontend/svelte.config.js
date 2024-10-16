import adapter from '@sveltejs/adapter-node';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://kit.svelte.dev/docs/integrations#preprocessors
	// for more information about preprocessors
	preprocess: vitePreprocess(),

	vitePlugin: {
		// set to true for defaults or customize with an object
		inspector: {
			toggleKeyCombo: 'meta-shift',
			showToggleButton: 'always',
			toggleButtonPos: 'bottom-left'
		}
	},

	kit: {
		adapter: adapter(),

		alias: {
			$components: './src/lib/components'
		}
	}
};

export default config;
