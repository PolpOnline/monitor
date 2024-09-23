import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import Icons from 'unplugin-icons/vite';
import Unfonts from 'unplugin-fonts/vite';

export default defineConfig({
	plugins: [
		sveltekit(),

		Icons({
			compiler: 'svelte',
			autoInstall: true
		}),

		Unfonts({
			// Fontsource API
			fontsource: {
				/**
				 * Fonts families lists
				 */
				families: [
					// families can be either strings (load default font set)
					'JetBrains Mono Variable'
				]
			}
		})
	]
});
