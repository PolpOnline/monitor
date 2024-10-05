<script lang="ts">
	import type { Instant, Status, SystemData } from '../../../../../backend/bindings';
	import HeroiconsXMark20Solid from '~icons/heroicons/x-mark-20-solid';
	import HeroiconsCheck20Solid from '~icons/heroicons/check-20-solid';
	import { slide } from 'svelte/transition';
	import { cubicIn, cubicOut } from 'svelte/easing';
	import humanizeDuration from 'humanize-duration';

	export let colorMapText: Record<string, string>;
	export let data: SystemData;

	let transitionIn = { easing: cubicOut, duration: 300 };
	let transitionOut = { easing: cubicIn, duration: 300 };

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

	$: lastInstant = data.instants[data.instants.length - 1];
</script>

<div in:slide={transitionIn} out:slide={transitionOut}>
	<h2 class="text-lg {colorMapText[lastInstant.status]} my-1 flex items-center">
		{#if lastInstant.status === 'up'}
			<HeroiconsCheck20Solid class="mr-2 inline-block h-6 w-6 min-w-6" />
			Operational
		{:else if lastInstant.status === 'down'}
			<HeroiconsXMark20Solid class="mr-2 inline-block h-6 w-6 min-w-6" />
			Down
			{#if downTime}
				<br class="sm:hidden" />
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
</div>
