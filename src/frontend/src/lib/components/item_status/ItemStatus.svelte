<script lang="ts">
	import type { Instant, SystemData } from '$lib/bindings';
	import ItemStatusDropdown from './ItemStatusDropdown.svelte';
	import { page } from '$app/stores';
	import ItemStatusOperationalStatus from '$components/item_status/ItemStatusOperationalStatus.svelte';
	import ItemStatusGraph from '$components/item_status/ItemStatusGraph.svelte';
	import { invalidateAll } from '$app/navigation';
	import { DateTime, Duration } from 'luxon';
	import { onDestroy, onMount } from 'svelte';

	export let showDropdown: boolean = true;
	export let data: SystemData;

	$: currentPage = Number($page.url.searchParams.get('page')) || 0;

	let timeoutId: ReturnType<typeof setTimeout>;
	let intervalId: ReturnType<typeof setInterval>;

	function refresh(system: SystemData) {
		return () => {
			console.debug(`%cRefreshing "${system.name}"`, 'color: #00d5ff;');
			invalidateAll();
		};
	}

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
			refresh(system)();
			intervalId = setInterval(refresh(system), frequency.as('milliseconds'));
		}, firstRefreshFromNow.as('milliseconds'));

		console.debug(
			`%cScheduled refresh for "${system.name}": ${firstRefresh.toJSDate().toLocaleString()} \n` +
				`Later refreshes will be every ${frequency.as('minutes')} minutes`,
			'color: #00d5ff;'
		);

		return timeoutId;
	}

	onMount(() => {
		if (currentPage === 0) {
			autoRefreshSystem(data);
		}
	});

	onDestroy(() => {
		clearInterval(intervalId);
		clearTimeout(timeoutId);
	});

	let now = DateTime.now();
	onMount(() => {
		const interval = setInterval(() => {
			now = DateTime.now();
		}, 1000);
		return () => clearInterval(interval);
	});
</script>

<div class="relative h-full rounded-lg border p-3">
	{#if showDropdown}
		<div class="absolute right-3 top-3 mr-2 mt-2">
			<ItemStatusDropdown {data} />
		</div>
	{/if}

	<h1 class="mb-1 text-2xl font-bold">
		{data.name}
	</h1>

	{#if currentPage === 0}
		<ItemStatusOperationalStatus {data} {now} />
	{/if}

	<div class="mx-auto my-3 max-w-[800px]">
		<ItemStatusGraph {data} {now} {currentPage} />
	</div>
</div>
