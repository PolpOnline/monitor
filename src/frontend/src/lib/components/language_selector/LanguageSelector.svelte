<script lang="ts">
	import Check from 'lucide-svelte/icons/check';
	import ChevronsUpDown from 'lucide-svelte/icons/chevrons-up-down';
	import { tick } from 'svelte';
	// noinspection ES6UnusedImports
	import * as Command from '$components/ui/command';
	// noinspection ES6UnusedImports
	import * as Popover from '$components/ui/popover';
	import { Button } from '$components/ui/button';
	import { cn } from '$lib/utils.js';
	import type { ClassValue } from 'clsx';
	import { getTranslate, T } from '@tolgee/svelte';

	const { t } = getTranslate();

	let open = false;
	export let value = '';
	export let languages: { value: string; label: string }[] = [];

	$: selectedValue =
		languages.find((f) => f.value === value)?.label ?? $t('language_selector.select_a_language');

	// We want to refocus the trigger button when the user selects
	// an item from the list, so users can continue navigating the
	// rest of the form with the keyboard.
	function closeAndFocusTrigger(triggerId: string) {
		open = false;
		tick().then(() => {
			document.getElementById(triggerId)?.focus();
		});
	}

	function searchLanguageFn(value: string, search: string): number {
		return value.toLowerCase().includes(search.toLowerCase()) ? 1 : 0;
	}

	let className: ClassValue;
	// noinspection ReservedWordAsName
	export { className as class };
</script>

<Popover.Root bind:open let:ids>
	<Popover.Trigger asChild let:builder>
		<Button
			builders={[builder]}
			variant="outline"
			role="combobox"
			aria-expanded={open}
			class={cn(className, 'justify-between')}
		>
			{selectedValue}
			<ChevronsUpDown class="ml-2 h-4 w-4 shrink-0 opacity-50" />
		</Button>
	</Popover.Trigger>
	<Popover.Content class="w-[95%] p-0 md:w-1/2">
		<Command.Root filter={searchLanguageFn}>
			<Command.Input placeholder={$t('language_selector.search_language')} />
			<Command.List>
				<Command.Empty>
					<T keyName="language_selector.no_language_found" />
				</Command.Empty>
				<Command.Group>
					{#each languages as language}
						<Command.Item
							value={language.value}
							onSelect={(currentValue) => {
								value = currentValue;
								closeAndFocusTrigger(ids.trigger);
							}}
						>
							<Check class={cn('mr-2 h-4 w-4', value !== language.value && 'text-transparent')} />
							{language.label}
						</Command.Item>
					{/each}
				</Command.Group>
			</Command.List>
		</Command.Root>
	</Popover.Content>
</Popover.Root>
