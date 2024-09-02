<script lang="ts">
	import HeroiconsXMark20Solid from '~icons/heroicons/x-mark-20-solid';
	import HeroiconsExclamationTriangle20Solid from '~icons/heroicons/exclamation-triangle-20-solid';
	import HeroiconsCheck20Solid from '~icons/heroicons/check-20-solid';
	import type { Instant, Status, SystemData } from '$lib/types/items';
	import humanizeDuration from 'humanize-duration';
	// noinspection ES6UnusedImports
	import * as Tooltip from '$lib/components/ui/tooltip';

	export let data: SystemData;

	const colorMap = {
		ok: 'bg-green-500',
		warning: 'bg-yellow-500',
		error: 'bg-red-500'
	};

	const colorMapText = {
		ok: 'text-green-500',
		warning: 'text-yellow-500',
		error: 'text-red-500'
	};

	const colorMapBorder = {
		ok: 'border-green-700',
		warning: 'border-yellow-700',
		error: 'border-red-700'
	};

	const lastInstant = data.instants[data.instants.length - 1];

	let tooltipOpens: boolean[] = new Array(data.instants.length).fill(false);

	function clearTooltipsExcept(index: number) {
		tooltipOpens = tooltipOpens.map((_, i) => i === index);
	}

	const uptime =
		(data.instants.filter((instant) => instant.status === 'ok').length / data.instants.length) *
		100;

	function calculateDownTime(instants: Instant[], level: Status) {
		// back track from the most recent instant to find the first error
		for (const [index, instant] of instants.toReversed().entries()) {
			if (instant.status === level) {
				continue;
			}
			return humanizeDuration(
				// Difference between now and the most recent instant
				Date.now() - instants[index + 1].timestamp.getTime(),
				{ round: true }
			);
		}
	}
</script>

<div class="mx-4 rounded-lg border p-3">
	<h1 class="text-2xl font-bold">
		{data.name}
	</h1>

	<h2 class="text-lg {colorMapText[lastInstant.status]} flex items-center">
		{#if lastInstant.status === 'ok'}
			<HeroiconsCheck20Solid class="mr-2 inline-block h-6" />
			Operational
		{:else if lastInstant.status === 'warning'}
			<HeroiconsExclamationTriangle20Solid class="mr-2 inline-block h-6" />
			Not functioning properly (since {calculateDownTime(data.instants, 'warning')} ago)
		{:else if lastInstant.status === 'error'}
			<HeroiconsXMark20Solid class="mr-2 inline-block h-6" />
			Down (since {calculateDownTime(data.instants, 'error')} ago)
		{/if}
	</h2>

	<p class="text-sm text-gray-500">
		Last check: {lastInstant.timestamp.toLocaleString()} (checking every {humanizeDuration(
			data.frequency * 1000 * 60
		)})
	</p>

	<div class="mx-auto my-3 max-w-[700px]">
		<div class="flex h-[50px] justify-between">
			{#each data.instants as instant, i (instant.timestamp)}
				<Tooltip.Root
					openDelay={0}
					bind:open={tooltipOpens[i]}
					onOpenChange={() => clearTooltipsExcept(i)}
				>
					<Tooltip.Trigger
						class="mx-0.25 h-full rounded {colorMap[instant.status]} max-w-3"
						style="width: calc((100% / {data.instants.length}) - 2px)"
					/>
					<Tooltip.Content class="{colorMap[instant.status]} {colorMapBorder[instant.status]}">
						<Tooltip.Arrow class="{colorMapText[instant.status]} rounded-[2px]" />
						<div class="flex items-center">
							{#if instant.status === 'ok'}
								<HeroiconsCheck20Solid class="mr-2 inline-block h-6" />
							{:else if instant.status === 'warning'}
								<HeroiconsExclamationTriangle20Solid class="mr-2 inline-block h-6" />
							{:else if instant.status === 'error'}
								<HeroiconsXMark20Solid class="mr-2 inline-block h-6" />
							{/if}
							{instant.timestamp.toLocaleString()}
						</div>
					</Tooltip.Content>
				</Tooltip.Root>
			{/each}
		</div>

		<div class="flex justify-between text-gray-500">
			<span>
				{humanizeDuration(
					// Difference between now and the least recent instant
					Date.now() - data.instants[data.instants.length - 1].timestamp.getTime(),
					{ round: true, units: ['y', 'd', 'h', 'm'] }
				)} ago
			</span>

			<span class="absolute left-1/2 -translate-x-1/2">
				{uptime.toFixed(2)}% uptime
			</span>

			<span>
				{humanizeDuration(
					// Difference between now and the most recent instant
					Date.now() - data.instants[0].timestamp.getTime(),
					{ round: true, units: ['y', 'd', 'h', 'm'] }
				)} ago
			</span>
		</div>
	</div>
</div>
