<script lang="ts">
	import { Input } from '$lib/components/ui/input/index.js';
	import { Label } from '$lib/components/ui/label/index.js';
	import DatePicker from './date_picker/DatePicker.svelte';
	import {
		type DateValue,
		getLocalTimeZone,
		Time,
		toCalendarDateTime
	} from '@internationalized/date';

	export let value: string | undefined;

	const now = new Date();

	let hours = now.getMinutes() > 30 ? now.getHours() + 1 : now.getHours();
	let minutes = 0;
	let seconds = 0;

	let date: DateValue | undefined;

	$: time = new Time(hours, minutes, seconds);

	$: completeDate = date ? toCalendarDateTime(date, time) : undefined;

	$: value = completeDate?.toDate(getLocalTimeZone()).toISOString();
</script>

<div class="flex flex-col items-center">
	<DatePicker bind:value={date} class="w-full" />
	{#if date}
		<div class="mt-2 flex w-full content-center items-center gap-2">
			<div class="w-1/3">
				<Label for="hours">HH (0-24)</Label>
				<div class="flex w-full items-center gap-2">
					<Input bind:value={hours} type="number" placeholder="HH" min="0" max="24" /> :
				</div>
			</div>
			<div class="w-1/3">
				<Label for="minutes">MM (0-59)</Label>
				<div class="flex w-full items-center gap-2">
					<Input bind:value={minutes} type="number" placeholder="MM" min="0" max="59" /> :
				</div>
			</div>
			<div class="w-1/3">
				<Label for="seconds">SS (0-59)</Label>
				<Input bind:value={seconds} type="number" placeholder="SS" min="0" max="59" />
			</div>
		</div>
	{/if}
</div>
