import { sveltekit } from '@sveltejs/kit/vite';
import type { FontsourceFontFamily } from 'unplugin-fonts/types';
import Unfonts from 'unplugin-fonts/vite';
import Icons from 'unplugin-icons/vite';
import { defineConfig } from 'vite';
import tailwindcss from '@tailwindcss/vite';

export default defineConfig({
	plugins: [
		tailwindcss(),
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
					{
						name: 'JetBrains Mono Variable',
						subset: 'latin-ext'
					} as FontsourceFontFamily
				]
			}
		})
	]
});
