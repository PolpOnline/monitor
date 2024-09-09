<script lang="ts">
	import '../app.pcss';
	import Navbar from '$lib/components/Navbar.svelte';
	import favicon from '$lib/images/favicon.svg';
	import { ModeWatcher } from 'mode-watcher';
	import type { LayoutData } from './$types';

	import { fly } from 'svelte/transition';
	import { cubicIn, cubicOut } from 'svelte/easing';
	import { afterNavigate, beforeNavigate } from '$app/navigation';

	import { title } from '$components/stores/title.store';
	import Loader from '$components/Loader.svelte';

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
</script>

<svelte:head>
	<link href={favicon} rel="icon" type="image/svg+xml" />
	<title>{$title}</title>
</svelte:head>

<div data-vaul-drawer-wrapper>
	<Navbar loginStatus={data.loginStatus} />

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
