<script lang="ts">
	import { Calendar as CalendarPrimitive } from 'bits-ui';
	import { cn } from '$lib/utils.js';

	type $$Props = CalendarPrimitive.Props;

	type $$Events = CalendarPrimitive.Events;

	export let value: $$Props['value'] = undefined;
	export let placeholder: $$Props['placeholder'] = undefined;
	export let weekdayFormat: $$Props['weekdayFormat'] = 'short';

	let className: $$Props['class'] = undefined;
	export { className as class };
</script>

<CalendarPrimitive.Root
	{...$$restProps}
	bind:placeholder
	bind:value
	class={cn("p-3", className)}
	let:months
	let:weekdays
	on:keydown
	{weekdayFormat}
>
	<Calendar.Header>
		<Calendar.PrevButton />
		<Calendar.Heading />
		<Calendar.NextButton />
	</Calendar.Header>
	<Calendar.Months>
		{#each months as month}
			<Calendar.Grid>
				<Calendar.GridHead>
					<Calendar.GridRow class="flex">
						{#each weekdays as weekday}
							<Calendar.HeadCell>
								{weekday.slice(0, 2)}
							</Calendar.HeadCell>
						{/each}
					</Calendar.GridRow>
				</Calendar.GridHead>
				<Calendar.GridBody>
					{#each month.weeks as weekDates}
						<Calendar.GridRow class="mt-2 w-full">
							{#each weekDates as date}
								<Calendar.Cell {date}>
									<Calendar.Day {date} month={month.value} />
								</Calendar.Cell>
							{/each}
						</Calendar.GridRow>
					{/each}
				</Calendar.GridBody>
			</Calendar.Grid>
		{/each}
	</Calendar.Months>
</CalendarPrimitive.Root>
