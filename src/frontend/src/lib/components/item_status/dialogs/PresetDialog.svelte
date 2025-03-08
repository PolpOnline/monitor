<script lang="ts">
	// noinspection ES6UnusedImports
	import * as Dialog from '$components/ui/dialog';
	// noinspection ES6UnusedImports
	import * as Select from '$components/ui/select';
	import { Button } from '$components/ui/form';
	import CopyableTextarea from '$components/CopyableTextarea.svelte';
	import { getTranslate, T } from '@tolgee/svelte';
	import type { components } from '$lib/api/schema';
	import type { Snippet } from 'svelte';
	import { getPresets, presets } from './preset_gen/presetGen';

	let {
		children,
		targetSystemData
	}: {
		children: Snippet;
		targetSystemData: components['schemas']['SystemData'];
	} = $props();

	let open = $state(false);

	const { t } = getTranslate();

	const presetMap = getPresets(targetSystemData);

	let value = $state('');

	const triggerContent = $derived(
		presets.find((f) => f.value === value)?.label ?? $t('preset_dialog.select_a_preset')
	);
</script>

<Dialog.Root bind:open>
	<Dialog.Trigger class="contents">
		{@render children()}
	</Dialog.Trigger>
	<Dialog.Content>
		<Dialog.Header>
			<Dialog.Title>
				<T keyName="preset_dialog.title" />
			</Dialog.Title>

			<div class="mt-4!">
				<Select.Root bind:value type="single">
					<Select.Trigger class="w-full">
						{triggerContent}
					</Select.Trigger>
					<Select.Content>
						<Select.Group>
							{#each presets as preset (preset.value)}
								<Select.Item value={preset.value} label={preset.label}>
									{preset.label}
								</Select.Item>
							{/each}
						</Select.Group>
					</Select.Content>
				</Select.Root>
			</div>

			{#if value}
				<CopyableTextarea value={presetMap[value]} class="h-[320px]" />
			{/if}

			<Dialog.Footer>
				<Button onclick={() => (open = false)} class="w-full">
					<T keyName="close" />
				</Button>
			</Dialog.Footer>
		</Dialog.Header>
	</Dialog.Content>
</Dialog.Root>
