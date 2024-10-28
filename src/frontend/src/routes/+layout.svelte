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

	import { fly } from 'svelte/transition';
	import { cubicIn, cubicOut } from 'svelte/easing';
	import { afterNavigate, beforeNavigate } from '$app/navigation';

	import { title } from '$components/stores/title.store';
	import Loader from '$components/Loader.svelte';
	import enLang from '$lib/i18n/en.json';
	import itLang from '$lib/i18n/it.json';

	export let data: LayoutData;

	// Page transition
	const duration = 300;
	const delay = duration + 100;
	const y = 10;

	const transitionIn = { easing: cubicOut, y, duration, delay };
	const transitionOut = { easing: cubicIn, y: -y, duration };

	let isLoading = false;

	// Show loader only when navigating between internal pages
	beforeNavigate(({ to }) => (isLoading = !!to?.route.id));
	afterNavigate(() => (isLoading = false));

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
	<div>
		<Toaster richColors theme={$mode} />

		<Navbar loginStatus={data.loginStatus} loggedInEmail={data.loggedInEmail} />

		<ModeWatcher defaultMode={'dark'} />

		{#if isLoading}
			<Loader />
		{/if}

		{#key data.pathname}
			<div in:fly={transitionIn} out:fly={transitionOut}>
				<slot />
			</div>
		{/key}
	</div>
</TolgeeProvider>
