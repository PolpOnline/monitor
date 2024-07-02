<script lang="ts">
	import '../app.pcss';
	import Navbar from '$lib/components/Navbar.svelte';
	import favicon from '$lib/images/favicon.svg';
</script>

<svelte:head>
	<link href={favicon} rel="icon" type="image/svg+xml" />
</svelte:head>

<Navbar />

<main>
	<slot />
</main>
