<script lang="ts">
	import '../app.pcss';
	import 'unfonts.css';
	import Navbar from '$lib/components/Navbar.svelte';
	import favicon from '$lib/images/favicon.svg';
	import { mode, ModeWatcher } from 'mode-watcher';
	import type { LayoutData } from './$types';
	import { Toaster } from '$lib/components/ui/sonner';
	import {
		DevTools,
		FormatSimple,
		LanguageDetector,
		LanguageStorage,
		Tolgee,
		TolgeeProvider
	} from '@tolgee/svelte';
	import { TooltipProvider } from '$lib/components/ui/tooltip';

	import { fly } from 'svelte/transition';
	import { cubicIn, cubicOut } from 'svelte/easing';

	import { title } from '$components/stores/title.store';
	import enLang from '$lib/i18n/en.json';
	import itLang from '$lib/i18n/it.json';
	import type { Snippet } from 'svelte';
	import { ProgressBar } from '@prgm/sveltekit-progress-bar';

	let { data, children }: { data: LayoutData; children: Snippet } = $props();

	// Page transition
	const duration = 300;
	const delay = duration + 100;
	const y = 10;

	const transitionIn = { easing: cubicOut, y, duration, delay };
	const transitionOut = { easing: cubicIn, y: -y, duration };

	const tolgee = Tolgee()
		.use(DevTools())
		.use(FormatSimple())
		.use(LanguageDetector())
		.use(LanguageStorage())
		.init({
			availableLanguages: ['en', 'it'],
			defaultLanguage: 'en',
			staticData: {
				en: enLang,
				it: itLang
			},
			apiUrl: import.meta.env.VITE_TOLGEE_API_URL,
			apiKey: import.meta.env.VITE_TOLGEE_API_KEY
		});
</script>

<svelte:head>
	<link href={favicon} rel="icon" type="image/svg+xml" />
	<title>{$title}</title>
</svelte:head>

<TolgeeProvider {tolgee}>
	<TooltipProvider>
		<div>
			<Toaster richColors theme={$mode} />

			<ProgressBar class="text-white" zIndex={100} />

			<Navbar loginStatus={data.loginStatus} loggedInEmail={data.loggedInEmail} />

			<ModeWatcher defaultMode={'dark'} />

			{#key data.pathname}
				<div in:fly={transitionIn} out:fly={transitionOut}>
					{@render children()}
				</div>
			{/key}
		</div>
	</TooltipProvider>
</TolgeeProvider>
