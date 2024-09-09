<script lang="ts">
	import HeroiconsXMark20Solid from '~icons/heroicons/x-mark-20-solid';
	import HeroiconsCheck20Solid from '~icons/heroicons/check-20-solid';
	import humanizeDuration from 'humanize-duration';
	// noinspection ES6UnusedImports
	import * as Tooltip from '$components/ui/tooltip';
	import type { Instant, Status, SystemData } from '../../../../../backend/bindings';
	import ItemStatusDropdown from './ItemStatusDropdown.svelte';

	export let data: SystemData;

	const colorMap = {
		up: 'bg-green-500',
		down: 'bg-red-500',
		untracked: 'bg-gray-500'
	};

	const colorMapText = {
		up: 'text-green-500',
		down: 'text-red-500',
		untracked: 'text-gray-500'
	};

	const colorMapBorder = {
		up: 'border-green-700',
		down: 'border-red-700',
		untracked: 'border-gray-700'
	};

	$: lastInstant = data.instants[data.instants.length - 1];

	let tooltipOpens: boolean[] = new Array(data.instants.length).fill(false);

	function clearTooltipsExcept(index: number) {
		tooltipOpens = tooltipOpens.map((_, i) => i === index);
	}

	$: uptime =
		(data.instants.filter((instant) => instant.status === 'up').length /
			data.instants.filter((instant) => instant.status !== 'untracked').length) *
		100;

	function calculateDownTime(instants: Instant[], level: Status) {
		// back track from the most recent instant to find the first error
		for (const [index, instant] of instants.toReversed().entries()) {
			if (instant.status === level) {
				continue;
			}
			return humanizeDuration(
				// Difference between now and the most recent instant
				Date.now() - new Date(instants[instants.length - index - 1].expected_timestamp).getTime(),
				{ round: true, units: ['y', 'd', 'h', 'm'] }
			);
		}
	}

	$: downTime = calculateDownTime(data.instants, 'down');
	$: firstTime = humanizeDuration(
		// Difference between now and the least recent instant
		Date.now() - new Date(data.instants[0].expected_timestamp).getTime(),
		{ round: true, units: ['y', 'd', 'h', 'm'], largest: 2 }
	);
	$: lastTime = humanizeDuration(
		// Difference between now and the most recent instant
		Date.now() - new Date(data.instants[data.instants.length - 1].expected_timestamp).getTime(),
		{ round: true, units: ['y', 'd', 'h', 'm'], largest: 2 }
	);
</script>

<div class="relative mx-4 rounded-lg border p-3">
	<div class="absolute right-3 top-3 mr-2 mt-2">
		<ItemStatusDropdown systemId={data.id} />
	</div>

	<h1 class="mb-1 text-2xl font-bold">
		{data.name}
	</h1>

	<h2 class="text-lg {colorMapText[lastInstant.status]} my-1 flex items-center">
		{#if lastInstant.status === 'up'}
			<HeroiconsCheck20Solid class="mr-2 inline-block h-6 w-6 min-w-6" />
			Operational
		{:else if lastInstant.status === 'down'}
			<HeroiconsXMark20Solid class="mr-2 inline-block h-6 w-6 min-w-6" />
			Down
			<br class="sm:hidden" />
			{#if downTime}
				(for
				{downTime})
			{/if}
		{/if}
	</h2>

	<p class="mt-1 text-sm text-gray-500">
		Last check: {new Date(lastInstant.expected_timestamp).toLocaleString()} (checking every {humanizeDuration(
			data.frequency * 1000 * 60
		)})
	</p>

	<div class="mx-auto my-3 max-w-[700px]">
		<div class="flex h-[50px] justify-between">
			{#each data.instants as instant, i (instant.expected_timestamp)}
				<Tooltip.Root
					openDelay={0}
					bind:open={tooltipOpens[i]}
					onOpenChange={() => clearTooltipsExcept(i)}
				>
					<Tooltip.Trigger
						class="mx-0.25 h-full rounded {colorMap[instant.status]} max-w-3"
						style="width: calc((100% / {data.instants.length}) - 2px)"
					>
						<button
							class="h-full w-full cursor-default"
							on:mouseenter={() => {
								clearTooltipsExcept(i);
							}}
							on:mouseleave={() => {
								tooltipOpens[i] = false;
							}}
						/>
					</Tooltip.Trigger>
					<Tooltip.Content class="{colorMap[instant.status]} {colorMapBorder[instant.status]}">
						<Tooltip.Arrow class="{colorMapText[instant.status]} rounded-[2px]" />
						<div class="flex items-center">
							{#if instant.status !== 'untracked'}
								{#if instant.status === 'up'}
									<HeroiconsCheck20Solid class="mr-2 inline-block h-6" />
								{:else if instant.status === 'down'}
									<HeroiconsXMark20Solid class="mr-2 inline-block h-6" />
								{/if}

								Expected: {new Date(instant.expected_timestamp).toLocaleString()}

								{#if instant.timestamp}
									<br />
									Actual: {new Date(instant.timestamp).toLocaleString()}
								{/if}
							{:else}
								<span>Untracked</span>
							{/if}
						</div>
					</Tooltip.Content>
				</Tooltip.Root>
			{/each}
		</div>

		<div class="mt-1 flex justify-between text-gray-500">
			<span>
				{firstTime} ago
			</span>

			<span class="sm:absolute sm:left-1/2 sm:-translate-x-1/2">
				{uptime.toFixed(2)}% uptime
			</span>

			<span>
				{lastTime} ago
			</span>
		</div>
	</div>
</div>
