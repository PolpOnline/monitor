<script lang="ts">
	import humanizeDuration from 'humanize-duration';
	// noinspection ES6UnusedImports
	import * as Tooltip from '$components/ui/tooltip';
	import type { SystemData } from '../../../../../backend/bindings';
	import HeroiconsXMark20Solid from '~icons/heroicons/x-mark-20-solid';
	import HeroiconsCheck20Solid from '~icons/heroicons/check-20-solid';

	export let data: SystemData;
	export let colorMap: Record<string, string>;
	export let colorMapText: Record<string, string>;
	export let colorMapBorder: Record<string, string>;

	let tooltipOpens: boolean[] = new Array(data.instants.length).fill(false);

	function clearTooltipsExcept(index: number) {
		tooltipOpens = tooltipOpens.map((_, i) => i === index);
	}

	$: uptime = ((data.instants.filter((instant) => instant.status === 'up').length /
		data.instants.filter((instant) => instant.status !== 'untracked').length) *
		100) as number;

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

<div class="mt-1 grid grid-flow-col grid-cols-3 text-gray-500">
	<div class="text-left">
		{firstTime} ago
	</div>

	<div class="text-center">
		{#if !isNaN(uptime)}
			{uptime.toFixed(2)}%
		{:else}
			Unknown
		{/if}
		uptime
	</div>

	<div class="text-right">
		{lastTime} ago
	</div>
</div>
