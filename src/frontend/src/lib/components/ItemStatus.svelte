<script lang="ts">
	import HeroiconsXMark20Solid from '~icons/heroicons/x-mark-20-solid';
	import HeroiconsCheck20Solid from '~icons/heroicons/check-20-solid';
	import humanizeDuration from 'humanize-duration';
	// noinspection ES6UnusedImports
	import * as Tooltip from '$lib/components/ui/tooltip';
	import type { Instant, SystemData, Status } from '../../../../backend/bindings/index';

	export let data: SystemData;

	const colorMap = {
		up: 'bg-green-500',
		down: 'bg-red-500'
	};

	const colorMapText = {
		up: 'text-green-500',
		down: 'text-red-500'
	};

	const colorMapBorder = {
		up: 'border-green-700',
		down: 'border-red-700'
	};

	const lastInstant = data.instants[data.instants.length - 1];

	let tooltipOpens: boolean[] = new Array(data.instants.length).fill(false);

	function clearTooltipsExcept(index: number) {
		tooltipOpens = tooltipOpens.map((_, i) => i === index);
	}

	const uptime =
		(data.instants.filter((instant) => instant.status === 'up').length / data.instants.length) *
		100;

	function calculateDownTime(instants: Instant[], level: Status) {
		// back track from the most recent instant to find the first error
		for (const [index, instant] of instants.toReversed().entries()) {
			if (instant.status === level) {
				continue;
			}
			return humanizeDuration(
				// Difference between now and the most recent instant
				Date.now() - instants[instants.length - index - 1].timestamp.getTime(),
				{ round: true }
			);
		}
	}
</script>

<div class="mx-4 rounded-lg border p-3">
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
			(for
			{calculateDownTime(data.instants, 'down')})
		{/if}
	</h2>

	<p class="mt-1 text-sm text-gray-500">
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
					>
						<button
							class="h-full w-full"
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
							{#if instant.status === 'up'}
								<HeroiconsCheck20Solid class="mr-2 inline-block h-6" />
							{:else if instant.status === 'down'}
								<HeroiconsXMark20Solid class="mr-2 inline-block h-6" />
							{/if}
							{instant.timestamp.toLocaleString()}
						</div>
					</Tooltip.Content>
				</Tooltip.Root>
			{/each}
		</div>

		<div class="mt-1 flex justify-between text-gray-500">
			<span>
				{humanizeDuration(
					// Difference between now and the least recent instant
					Date.now() - data.instants[0].timestamp.getTime(),
					{ round: true, units: ['y', 'd', 'h', 'm'], largest: 2 }
				)} ago
			</span>

			<span class="sm:absolute sm:left-1/2 sm:-translate-x-1/2">
				{uptime.toFixed(2)}% uptime
			</span>

			<span>
				{humanizeDuration(
					// Difference between now and the most recent instant
					Date.now() - data.instants[data.instants.length - 1].timestamp.getTime(),
					{ round: true, units: ['y', 'd', 'h', 'm'], largest: 2 }
				)} ago
			</span>
		</div>
	</div>
</div>
