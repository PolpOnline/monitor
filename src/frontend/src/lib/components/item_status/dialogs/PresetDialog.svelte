<script lang="ts">
	// noinspection ES6UnusedImports
	import * as Dialog from '$components/ui/dialog';
	// noinspection ES6UnusedImports
	import * as Select from '$components/ui/select';
	import { Button } from '$components/ui/form';
	import { presetDialogOpen, targetSystemData } from '$components/stores/popovers.store';
	import CopyableTextarea from '$components/CopyableTextarea.svelte';
	import { API_URL } from '$lib/api/api';
	import { getTranslate, T } from '@tolgee/svelte';

	const { t } = getTranslate();

	const presets = [
		{
			value: 'mikrotik',
			label: 'Mikrotik RouterOS'
		}
	];

	const startsAtDateTime = $derived(
		$targetSystemData ? new Date($targetSystemData.starts_at) : undefined
	);

	const startsAtDate = $derived.by(() => {
		const startsAtDateTimeCalled = startsAtDateTime;

		if (!startsAtDateTimeCalled) {
			return undefined;
		}

		return startsAtDateTimeCalled
			.toLocaleDateString('en-US', { year: 'numeric', month: 'short', day: '2-digit' })
			.replace(',', '')
			.replaceAll(' ', '/')
			.toLowerCase();
	}); // Format: mmm/dd/yyyy (e.g. jan/01/2022)

	const startsAtTime = $derived.by(() => {
		const startsAtDateTimeCalled = startsAtDateTime;

		if (!startsAtDateTimeCalled) {
			return undefined;
		}

		return startsAtDateTimeCalled.toLocaleTimeString('en-GB', { hour12: false });
	}); // Format: 00:00:00

	const frequencyHours = $derived(
		$targetSystemData ? Math.floor($targetSystemData.frequency / 60) : undefined
	);
	const frequencyMinutes = $derived(
		$targetSystemData ? $targetSystemData.frequency % 60 : undefined
	);

	const presetMap = $derived.by(() => {
		return {
			mikrotik: `/system scheduler add name="ping_status" start-date="${startsAtDate}" start-time="${startsAtTime}" interval="${frequencyHours}:${frequencyMinutes}:00" \
on-event="/tool fetch url=\\"${API_URL}/ping_status/${$targetSystemData?.id}\\" \
mode=https http-method=post output=none"`
		} as Record<string, string>;
	});

	// let chosenPreset: (typeof presets)[0] | undefined = undefined;

	let value = $state('');

	const triggerContent = $derived(
		presets.find((f) => f.value === value)?.label ?? $t('preset_dialog.select_a_preset')
	);
</script>

<Dialog.Root bind:open={$presetDialogOpen}>
	<Dialog.Content>
		<Dialog.Header>
			<Dialog.Title>
				<T keyName="preset_dialog.title" />
			</Dialog.Title>

			<div class="!mt-4">
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
				<Button onclick={() => ($presetDialogOpen = false)} class="w-full">
					<T keyName="close" />
				</Button>
			</Dialog.Footer>
		</Dialog.Header>
	</Dialog.Content>
</Dialog.Root>
