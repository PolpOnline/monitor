<script lang="ts">
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { T } from '@tolgee/svelte';

	const {
		defaultValue = { hours: 0, minutes: 0 }
	}: {
		defaultValue: {
			hours: number;
			minutes: number;
		};
	} = $props();

	let hours = $state(defaultValue.hours);
	let minutes = $state(defaultValue.minutes);

	// noinspection JSUnusedGlobalSymbols
	const value: number = $derived((hours || 0) * 60 + (minutes || 0));

	export { value };
</script>

<div class="mt-2 flex w-full content-center items-center gap-2">
	<div class="w-1/2">
		<Label for="hours" class="mb-1">
			<T keyName="duration_picker.hours" />
		</Label>
		<div class="flex w-full items-center gap-2">
			<Input bind:value={hours} type="number" placeholder="HH" min="0" max="24" /> :
		</div>
	</div>
	<div class="w-1/2">
		<Label for="minutes" class="mb-1">
			<T keyName="duration_picker.minutes" />
		</Label>
		<div class="flex w-full items-center gap-2">
			<Input bind:value={minutes} type="number" placeholder="MM" min="0" max="59" />
		</div>
	</div>
</div>
