<script lang="ts">
	import humanizeDuration from 'humanize-duration';
	// noinspection ES6UnusedImports
	import * as Tooltip from '$components/ui/tooltip';
	import type { SystemData } from '$lib/bindings';
	import HeroiconsXMark20Solid from '~icons/heroicons/x-mark-20-solid';
	import HeroiconsCheck20Solid from '~icons/heroicons/check-20-solid';
	import { DateTime } from 'luxon';
	import { colorMap, colorMapBorder, colorMapText } from './index';

	export let data: SystemData;
	export let now: DateTime;

	let tooltipOpens: boolean[] = new Array(data.instants.length).fill(false);

	function clearTooltipsExcept(index: number) {
		tooltipOpens = tooltipOpens.map((_, i) => i === index);
	}

	function calculateUptime(): number {
		const upInstants = data.instants.filter((instant) => instant.status === 'up').length;
		const validInstants = data.instants.filter((instant) => instant.status !== 'untracked').length;

		return (upInstants / validInstants) * 100;
	}

	$: uptime = calculateUptime();

	$: firstInstantExpected = DateTime.fromISO(data.instants[0].expected_timestamp);
	$: lastInstantExpected = DateTime.fromISO(
		data.instants[data.instants.length - 1].expected_timestamp
	);

	const durationParams: humanizeDuration.Options = {
		round: true,
		units: ['y', 'd', 'h', 'm'],
		largest: 2
	};

	$: firstTime = humanizeDuration(
		// Difference between now and the least recent instant
		now.diff(firstInstantExpected).as('milliseconds'),
		durationParams
	);

	$: lastTime = humanizeDuration(
		// Difference between now and the most recent instant
		now.diff(lastInstantExpected).as('milliseconds'),
		durationParams
	);
</script>

<div class="flex h-[50px] justify-between">
	{#each data.instants as instant, i (instant.expected_timestamp)}
		<Tooltip.Root
			openDelay={0}
			bind:open={tooltipOpens[i]}
			onOpenChange={() => clearTooltipsExcept(i)}
		>
			<div
				class="mx-0.25 h-full rounded {colorMap[instant.status]} max-w-3 cursor-default"
				style="width: calc((100% / {data.instants.length}) - 2px)"
				on:mouseenter={() => {
					clearTooltipsExcept(i);
				}}
				on:mouseleave={() => {
					tooltipOpens[i] = false;
				}}
				role="button"
				tabindex={-1}
			>
				<Tooltip.Trigger class="h-full w-full cursor-default" aria-labelledby={'instant' + i} />
			</div>
			<Tooltip.Content class="{colorMap[instant.status]} {colorMapBorder[instant.status]}">
				<Tooltip.Arrow class="{colorMapText[instant.status]} rounded-[2px]" />
				<div class="flex items-center" id={'instant' + i}>
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
