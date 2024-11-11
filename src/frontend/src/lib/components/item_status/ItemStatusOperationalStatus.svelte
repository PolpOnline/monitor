<script lang="ts">
	import type { SystemData } from '$lib/bindings';
	import HeroiconsXMark20Solid from '~icons/heroicons/x-mark-20-solid';
	import HeroiconsCheck20Solid from '~icons/heroicons/check-20-solid';
	import { slide, type SlideParams } from 'svelte/transition';
	import { T } from '@tolgee/svelte';
	import { cubicIn, cubicOut } from 'svelte/easing';
	import humanizeDuration from 'humanize-duration';
	import { language } from '$lib/components/stores/language.store';
	import { DateTime } from 'luxon';
	import { colorMapText } from './index';

	export let data: SystemData;
	export let now: DateTime;

	let transitionIn: SlideParams = { easing: cubicOut, duration: 300 };
	let transitionOut: SlideParams = { easing: cubicIn, duration: 300 };

	$: firstInstant = data.instants[0];
	$: lastInstant = data.instants[data.instants.length - 1];

	$: downTime = (() => {
		// System is up, no need to calculate the downtime
		if (lastInstant.status !== 'down') {
			return;
		}

		// back track from the most recent instant to find the first error
		const mostRecentUp = data.instants.findLastIndex((instant) => instant.status === 'up');

		if (mostRecentUp === -1) {
			return;
		}

		const mostRecentDown = data.instants[mostRecentUp + 1];

		const lastInstantTime = DateTime.fromISO(mostRecentDown.expected_timestamp);

		return humanizeDuration(
			// Difference between now and the most recent instant
			now.diff(lastInstantTime).as('milliseconds'),
			{ round: true, units: ['y', 'd', 'h', 'm'], language: $language }
		);
	})();
</script>

<div in:slide={transitionIn} out:slide={transitionOut}>
	<h2 class="text-lg {colorMapText[lastInstant.status]} my-1 flex items-center">
		{#if lastInstant.status === 'up'}
			<HeroiconsCheck20Solid class="mr-2 inline-block h-6 w-6 min-w-6" />
			<T keyName="operational" />
		{:else if lastInstant.status === 'down'}
			<HeroiconsXMark20Solid class="mr-2 inline-block h-6 w-6 min-w-6" />

			<T keyName="down" />

			{#if downTime}
				<br class="sm:hidden" />
				<T keyName="down_for" params={{ time: downTime }} />
			{/if}
		{/if}
	</h2>

	<p class="mt-1 text-sm text-gray-500">
		<T
			keyName="last_check"
			params={{ time: new Date(lastInstant.expected_timestamp).toLocaleString() }}
		/>

		<T
			keyName="checking_every"
			params={{ frequency: humanizeDuration(data.frequency * 1000 * 60, { language: $language }) }}
		/>
	</p>
</div>
