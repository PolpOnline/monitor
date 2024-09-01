<script lang="ts">
	import ItemStatus from '$components/ItemStatus.svelte';
	import type { PageData } from './$types';

	export let data: PageData;
</script>

<svelte:head>
	<title>Monitor</title>
</svelte:head>

{#each data.systems as system}
	<div class="my-3">
		<ItemStatus data={system} />
	</div>
{/each}
