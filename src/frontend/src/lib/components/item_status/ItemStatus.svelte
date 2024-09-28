<script lang="ts">
	import type { Instant, SystemData } from '../../../../../backend/bindings';
	import ItemStatusDropdown from './ItemStatusDropdown.svelte';
	import { page } from '$app/stores';
	import ItemStatusOperationalStatus from '$components/item_status/ItemStatusOperationalStatus.svelte';
	import ItemStatusGraph from '$components/item_status/ItemStatusGraph.svelte';
	import { invalidateAll } from '$app/navigation';
	import { DateTime, Duration } from 'luxon';
	import { onMount, onDestroy } from 'svelte';

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

	let timeoutId: ReturnType<typeof setTimeout>;

	function autoRefreshSystem(system: SystemData) {
		const lastInstantRaw: Instant = system.instants[system.instants.length - 1];
		// add 5 seconds to the last instant time to avoid refreshing too soon
		const lastInstant = DateTime.fromISO(lastInstantRaw.expected_timestamp).plus(
			Duration.fromObject({ second: 5 })
		);

		const frequency = Duration.fromObject({ minute: system.frequency });

		const firstRefresh = lastInstant.plus(frequency);
		const firstRefreshFromNow = firstRefresh.diffNow();

		timeoutId = setTimeout(() => {
			invalidateAll();
			setInterval(invalidateAll, frequency.as('milliseconds'));
		}, firstRefreshFromNow.as('milliseconds'));

		// console.log(
		// 	'Scheduled refresh for',
		// 	firstRefresh.toJSDate().toLocaleString(),
		// 	', Later refreshes will be every',
		// 	frequency.as('minutes'),
		// 	'minutes'
		// );

		return timeoutId;
	}

	onMount(() => {
		if (currentPage === 0) {
			autoRefreshSystem(data);
		}
	});

	onDestroy(() => {
		clearTimeout(timeoutId);
	});
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
