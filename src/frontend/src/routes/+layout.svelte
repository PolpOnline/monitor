<script lang="ts">
	import '../app.pcss';
	import Navbar from '$lib/components/Navbar.svelte';
	import favicon from '$lib/images/favicon.svg';
	import { ModeWatcher } from 'mode-watcher';

	import { title } from '$components/stores/title.store';

	export let data: PageData;
</script>

<svelte:head>
	<link href={favicon} rel="icon" type="image/svg+xml" />
	<title>{$title}</title>
</svelte:head>

<Navbar loginStatus={data.loginStatus} />

<ModeWatcher defaultMode={'dark'} />
<main>
	<slot />
</main>
