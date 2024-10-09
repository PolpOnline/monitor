<script lang="ts">
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';

	export let defaultValue: {
		hours: number;
		minutes: number;
	} = { hours: 0, minutes: 0 };
	export let value: number;

	let hours = defaultValue.hours;
	let minutes = defaultValue.minutes;

	$: value = (hours || 0) * 60 + (minutes || 0);
</script>

<div class="mt-2 flex w-full content-center items-center gap-2">
	<div class="w-1/2">
		<Label for="hours">HH (0-24)</Label>
		<div class="flex w-full items-center gap-2">
			<Input bind:value={hours} type="number" placeholder="HH" min="0" max="24" /> :
		</div>
	</div>
	<div class="w-1/2">
		<Label for="minutes">MM (0-59)</Label>
		<div class="flex w-full items-center gap-2">
			<Input bind:value={minutes} type="number" placeholder="MM" min="0" max="59" />
		</div>
	</div>
</div>
