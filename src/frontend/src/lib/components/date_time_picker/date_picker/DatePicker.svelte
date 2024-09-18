<script lang="ts">
	import CalendarIcon from 'lucide-svelte/icons/calendar';
	import { DateFormatter, type DateValue, getLocalTimeZone, today } from '@internationalized/date';
	import { cn } from '$lib/utils.js';
	import { Button } from '$lib/components/ui/button/index.js';
	import { Calendar } from '$lib/components/ui/calendar/index.js';
	// noinspection ES6UnusedImports
	import * as Popover from '$lib/components/ui/popover/index.js';
	import { type ClassValue } from 'clsx';

	const df = new DateFormatter('en-US', {
		dateStyle: 'long'
	});

	export let value: DateValue = today(getLocalTimeZone());

	let className: ClassValue = '';
	// noinspection ReservedWordAsName
	export { className as class };
</script>

<Popover.Root>
	<Popover.Trigger asChild let:builder>
		<Button
			builders={[builder]}
			class={cn(
				'justify-start text-left font-normal',
				!value && 'text-muted-foreground',
				className
			)}
			variant="outline"
		>
			<CalendarIcon class="mr-2 h-4 w-4" />
			{value ? df.format(value.toDate(getLocalTimeZone())) : 'Pick a date'}
		</Button>
	</Popover.Trigger>
	<Popover.Content class="w-auto p-0">
		<Calendar bind:value initialFocus placeholder={today(getLocalTimeZone())} />
	</Popover.Content>
</Popover.Root>
