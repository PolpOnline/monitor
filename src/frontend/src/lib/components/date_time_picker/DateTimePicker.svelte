<script lang="ts">
	import { Input } from '$lib/components/ui/input/index.js';
	import { Label } from '$lib/components/ui/label/index.js';
	import { today } from '@internationalized/date';
	import DatePicker from './date_picker/DatePicker.svelte';
	import {
		type DateValue,
		getLocalTimeZone,
		Time,
		toCalendarDateTime
	} from '@internationalized/date';
	import { T } from '@tolgee/svelte';

	const now = new Date();

	let hours = $state(now.getMinutes() > 30 ? now.getHours() + 1 : now.getHours());
	let minutes = $state(0);
	let seconds = $state(0);

	let date: DateValue = $state(today(getLocalTimeZone()));

	const time = $derived(new Time(hours, minutes, seconds));

	const completeDate = $derived(date ? toCalendarDateTime(date, time) : undefined);

	const value: string = $derived(
		completeDate ? completeDate.toDate(getLocalTimeZone()).toISOString() : ''
	);

	export { value };
</script>

<div class="flex flex-col items-center">
	<DatePicker bind:value={date} class="w-full" />
	{#if date}
		<div class="mt-2 flex w-full content-center items-center gap-2">
			<div class="w-1/3">
				<Label for="hours">
					<T keyName="date_time_picker.hours" />
				</Label>
				<div class="flex w-full items-center gap-2">
					<Input bind:value={hours} type="number" placeholder="HH" min="0" max="24" /> :
				</div>
			</div>
			<div class="w-1/3">
				<Label for="minutes">
					<T keyName="date_time_picker.minutes" />
				</Label>
				<div class="flex w-full items-center gap-2">
					<Input bind:value={minutes} type="number" placeholder="MM" min="0" max="59" /> :
				</div>
			</div>
			<div class="w-1/3">
				<Label for="seconds">
					<T keyName="date_time_picker.seconds" />
				</Label>
				<Input bind:value={seconds} type="number" placeholder="SS" min="0" max="59" />
			</div>
		</div>
	{/if}
</div>
