<script lang="ts">
	import CalendarIcon from 'lucide-svelte/icons/calendar';
	import { DateFormatter, type DateValue, getLocalTimeZone, today } from '@internationalized/date';
	import { cn } from '$lib/utils.js';
	import { buttonVariants } from '$lib/components/ui/button/index.js';
	import { Calendar } from '$lib/components/ui/calendar/index.js';
	// noinspection ES6UnusedImports
	import * as Popover from '$lib/components/ui/popover/index.js';
	import { type ClassValue } from 'clsx';
	import { getTranslate } from '@tolgee/svelte';

	const { t } = getTranslate();

	const df = new DateFormatter('en-US', {
		dateStyle: 'long'
	});

	let {
		value = today(getLocalTimeZone()),
		class: className = ''
	}: { value?: DateValue; class?: ClassValue } = $props();
</script>

<Popover.Root>
	<Popover.Trigger
		class={cn(
			buttonVariants({ variant: 'outline' }),
			'justify-start text-left font-normal',
			!value && 'text-muted-foreground',
			className
		)}
	>
		<CalendarIcon class="mr-2 h-4 w-4" />
		{value ? df.format(value.toDate(getLocalTimeZone())) : $t('date_time_picker.pick_a_date')}
	</Popover.Trigger>
	<Popover.Content class="w-auto p-0">
		<Calendar type="single" bind:value initialFocus placeholder={today(getLocalTimeZone())} />
	</Popover.Content>
</Popover.Root>
