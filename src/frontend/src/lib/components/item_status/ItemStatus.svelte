<script lang="ts">
	import type { SystemData } from '../../../../../backend/bindings';
	import ItemStatusDropdown from './ItemStatusDropdown.svelte';
	import { page } from '$app/stores';
	import ItemStatusOperationalStatus from '$components/item_status/ItemStatusOperationalStatus.svelte';
	import ItemStatusGraph from '$components/item_status/ItemStatusGraph.svelte';

	export let showDropdown: boolean = true;
	export let data: SystemData;

	$: currentPage = Number($page.url.searchParams.get('page')) || 0;

	const colorMap = {
		up: 'bg-green-500',
		down: 'bg-red-500',
		untracked: 'bg-gray-500'
	} as const;

	const colorMapText = {
		up: 'text-green-500',
		down: 'text-red-500',
		untracked: 'text-gray-500'
	} as const;

	const colorMapBorder = {
		up: 'border-green-700',
		down: 'border-red-700',
		untracked: 'border-gray-700'
	} as const;
</script>

<div class="relative my-3 rounded-lg border p-3">
	{#if showDropdown}
		<div class="absolute right-3 top-3 mr-2 mt-2">
			<ItemStatusDropdown {data} />
		</div>
	{/if}

	<h1 class="mb-1 text-2xl font-bold">
		{data.name}
	</h1>

	{#if currentPage === 0}
		<ItemStatusOperationalStatus {colorMapText} {data} />
	{/if}

	<div class="mx-auto my-3 max-w-[800px]">
		<ItemStatusGraph {data} {colorMap} {colorMapText} {colorMapBorder} />
	</div>
</div>
