<script lang="ts">
	import HeroiconsXMark20Solid from '~icons/heroicons/x-mark-20-solid';
	import HeroiconsExclamationTriangle20Solid from '~icons/heroicons/exclamation-triangle-20-solid';
	import HeroiconsCheck20Solid from '~icons/heroicons/check-20-solid';
	import type { SystemData } from '$lib/types/items';
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

	let tooltipOpens = new Array(data.instants.length).fill(false);

	function clearTooltipsExcept(index: number) {
		tooltipOpens = tooltipOpens.map((_, i) => i === index);
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
			Not functioning properly
		{:else if lastInstant.status === 'error'}
			<HeroiconsXMark20Solid class="mr-2 inline-block h-6" />
			Down
		{/if}
	</h2>

	<p class="text-sm text-gray-500">
		Last check: {lastInstant.timestamp.toLocaleString()} (checking every {humanizeDuration(
			data.frequency * 1000 * 60
		)})
	</p>

	<div class="mx-auto my-3 flex h-[50px] max-w-[700px] justify-between">
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
					<span class="flex items-center">
						{#if instant.status === 'ok'}
							<HeroiconsCheck20Solid class="mr-2 inline-block h-6" />
						{:else if instant.status === 'warning'}
							<HeroiconsExclamationTriangle20Solid class="mr-2 inline-block h-6" />
						{:else if instant.status === 'error'}
							<HeroiconsXMark20Solid class="mr-2 inline-block h-6" />
						{/if}
						{instant.timestamp.toLocaleString()}
					</span>
				</Tooltip.Content>
			</Tooltip.Root>
		{/each}
	</div>
</div>
