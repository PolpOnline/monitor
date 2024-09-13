<script lang="ts">
	// noinspection ES6UnusedImports
	import * as Dialog from '$components/ui/dialog';
	// noinspection ES6UnusedImports
	import * as Select from '$components/ui/select';
	import { Button } from '$components/ui/form';
	import { presetDialogOpen, targetSystemData } from '$components/stores/popovers.store';
	import CopyableTextarea from '$components/CopyableTextarea.svelte';
	import { API_URL } from '$lib/api/api';

	const presets = [
		{
			value: 'mikrotik',
			label: 'Mikrotik RouterOS'
		}
	];

	$: startsAtDateTime = $targetSystemData ? new Date($targetSystemData.starts_at) : undefined;

	$: startsAtDate = startsAtDateTime
		? startsAtDateTime
				.toLocaleDateString('en-US', { year: 'numeric', month: 'short', day: '2-digit' })
				.replace(',', '')
				.replaceAll(' ', '/')
				.toLowerCase()
		: undefined; // Format: mmm/dd/yyyy (e.g. jan/01/2022)

	$: startsAtTime = startsAtDateTime
		? startsAtDateTime.toLocaleTimeString('en-GB', { hour12: false })
		: undefined; // Format: 00:00:00

	$: frequencyHours = $targetSystemData ? Math.floor($targetSystemData.frequency / 60) : undefined;
	$: frequencyMinutes = $targetSystemData ? $targetSystemData.frequency % 60 : undefined;

	$: presetMap = {
		mikrotik: `/system scheduler add name="ping_status" start-date="${startsAtDate}" start-time="${startsAtTime}" interval="${frequencyHours}:${frequencyMinutes}:00" \
on-event="/tool fetch url=\\"${API_URL}/ping_status/${$targetSystemData?.id}\\" \
mode=https http-method=post output=none"`
	} as Record<string, string>;

	let chosenPreset: (typeof presets)[0] | undefined = undefined;
</script>

<Dialog.Root bind:open={$presetDialogOpen}>
	<Dialog.Trigger>
		<slot />
	</Dialog.Trigger>
	<Dialog.Content>
		<Dialog.Header>
			<Dialog.Title>Presets</Dialog.Title>

			<div class="!mt-4">
				<Select.Root bind:selected={chosenPreset} portal={null}>
					<Select.Trigger class="w-[180px]">
						<Select.Value placeholder="Select a preset" />
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

			{#if chosenPreset}
				<CopyableTextarea value={presetMap[chosenPreset.value]} class="h-[320px]" />
			{/if}

			<Dialog.Footer>
				<Button on:click={() => ($presetDialogOpen = false)}>Close</Button>
			</Dialog.Footer>
		</Dialog.Header>
	</Dialog.Content>
</Dialog.Root>
