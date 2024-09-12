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

	let hours = now.getHours();
	let minutes = now.getMinutes();
	let seconds = now.getSeconds();

	let date: DateValue | undefined;

	$: time = new Time(hours, minutes, seconds);

	$: completeDate = date ? toCalendarDateTime(date, time) : undefined;

	$: value = completeDate?.toDate(getLocalTimeZone()).toISOString();
</script>

<div>
	<DatePicker bind:value={date} />
	{#if date}
		<div class="row mt-2 flex w-[280px] content-center items-center gap-2">
			<div>
				<Label for="hours">HH (0-24)</Label>
				<div class="flex items-center gap-2">
					<Input bind:value={hours} type="number" placeholder="HH" min="0" max="24" /> :
				</div>
			</div>
			<div>
				<Label for="minutes">MM (0-59)</Label>
				<div class="flex items-center gap-2">
					<Input bind:value={minutes} type="number" placeholder="MM" min="0" max="59" /> :
				</div>
			</div>
			<div>
				<Label for="seconds">SS (0-59)</Label>
				<Input bind:value={seconds} type="number" placeholder="SS" min="0" max="59" />
			</div>
		</div>
	{/if}
</div>
