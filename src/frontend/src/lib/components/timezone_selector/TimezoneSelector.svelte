<script lang="ts">
	import Check from 'lucide-svelte/icons/check';
	import ChevronsUpDown from 'lucide-svelte/icons/chevrons-up-down';
	import { tick } from 'svelte';
	// noinspection ES6UnusedImports
	import * as Command from '$components/ui/command';
	// noinspection ES6UnusedImports
	import * as Popover from '$components/ui/popover';
	import { cn } from '$lib/utils.js';
	import type { ClassValue } from 'clsx';
	import { getTranslate, T } from '@tolgee/svelte';
	import { Button } from '$components/ui/form';

	const { t } = getTranslate();

	let open = $state(false);
	let {
		value = $bindable(''),
		timezones = [],
		class: className
	}: { value: string; timezones: { value: string; label: string }[]; class: ClassValue } = $props();
	let triggerRef = $state<HTMLButtonElement>(null!);

	const selectedValue = $derived(timezones.find((f) => f.value === value)?.label);

	// We want to refocus the trigger button when the user selects
	// an item from the list so users can continue navigating the
	// rest of the form with the keyboard.
	function closeAndFocusTrigger() {
		open = false;
		tick().then(() => {
			triggerRef.focus();
		});
	}

	function searchTimezoneFn(value: string, search: string): number {
		return value.toLowerCase().includes(search.toLowerCase()) ? 1 : 0;
	}
</script>

<Popover.Root bind:open>
	<Popover.Trigger bind:ref={triggerRef}>
		{#snippet child({ props })}
			<Button
				role="combobox"
				aria-expanded={open}
				variant="outline"
				{...props}
				class={cn(className, 'justify-between')}
			>
				{selectedValue || $t('timezone_selector.select_a_timezone')}
				<ChevronsUpDown class="ml-2 h-4 w-4 shrink-0 opacity-50" />
			</Button>
		{/snippet}
	</Popover.Trigger>
	<Popover.Content class="w-[95vw] p-0 md:w-[50vw]">
		<Command.Root filter={searchTimezoneFn} class="w-full">
			<Command.Input placeholder={$t('timezone_selector.search_timezone')} />
			<Command.List>
				<Command.Empty>
					<T keyName="timezone_selector.no_timezone_found" />
				</Command.Empty>
				<Command.Group>
					{#each timezones as timezone (timezone.value)}
						<Command.Item
							value={timezone.value}
							onSelect={() => {
								value = timezone.value;
								closeAndFocusTrigger();
							}}
						>
							<Check class={cn('mr-2 h-4 w-4', value !== timezone.value && 'text-transparent')} />
							{timezone.label}
						</Command.Item>
					{/each}
				</Command.Group>
			</Command.List>
		</Command.Root>
	</Popover.Content>
</Popover.Root>
