import adapter from '@sveltejs/adapter-node';
import { sveltekit } from '@sveltejs/kit/vite';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';
import tailwindcss from '@tailwindcss/vite';
import type { FontsourceFontFamily } from 'unplugin-fonts/types';
import Unfonts from 'unplugin-fonts/vite';
import Icons from 'unplugin-icons/vite';
import { defineConfig } from 'vite';
import devtoolsJson from 'vite-plugin-devtools-json';

export default defineConfig({
	plugins: [
		tailwindcss(),
		sveltekit({
			preprocess: vitePreprocess(),
			adapter: adapter(),
			alias: {
				$components: './src/lib/components',
				$lib: './src/lib'
			},
			inspector: {
				toggleKeyCombo: 'meta-shift',
				showToggleButton: 'always',
				toggleButtonPos: 'bottom-left'
			}
		}),
		Icons({ compiler: 'svelte', autoInstall: true }),
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
		}),
		devtoolsJson()
	]
});
