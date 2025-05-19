<script lang="ts">
	// noinspection ES6UnusedImports
	import * as Dialog from '$components/ui/dialog';
	import { Button } from '$components/ui/form';
	import { invalidateAll } from '$app/navigation';
	import { Input } from '$components/ui/input';
	import LineMdLoadingLoop from '~icons/line-md/loading-loop';
	import { toast } from 'svelte-sonner';
	import { getTranslate, T } from '@tolgee/svelte';
	import type { Snippet } from 'svelte';
	import type { components } from '$lib/api/schema';

	let {
		children,
		targetSystemData
	}: {
		children: Snippet;
		targetSystemData: components['schemas']['SystemData'];
	} = $props();

	const { t } = getTranslate();

	let newSystemName = $state('');
	let open = $state(false);

	async function editSystemName() {
		if (!targetSystemData) {
			return;
		}

		isLoading = true;

		newSystemName = newSystemName.trim();

		const res = await fetch(`/api/edit_system_name`, {
			method: 'PATCH',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({ id: targetSystemData.id, name: newSystemName })
		});

		if (res.ok) {
			toast.success($t('edit_system_name_dialog.success'));
		} else {
			toast.error($t('edit_system_name_dialog.fail', { error: await res.text() }));
		}

		await invalidateAll();

		isLoading = false;
		open = false;
	}

	let isLoading = $state(false);
</script>

<Dialog.Root bind:open>
	<Dialog.Trigger class="contents">
		{@render children()}
	</Dialog.Trigger>
	<Dialog.Content>
		<Dialog.Header>
			<Dialog.Title class="leading-6">
				<T keyName="edit_system_name_dialog.title" params={{ name: targetSystemData?.name }} />
			</Dialog.Title>

			<Input
				bind:value={newSystemName}
				class="my-3!"
				placeholder={$t('edit_system_name_dialog.new_system_name')}
			/>

			<Dialog.Footer>
				<Button class="mt-3 sm:mt-0" onclick={() => (open = false)} variant="secondary">
					<T keyName="cancel" />
				</Button>
				<Button class="mt-5 sm:mt-0" onclick={() => editSystemName()}>
					{#if !isLoading}
						<T keyName="save" />
					{:else}
						<LineMdLoadingLoop class="size-6" />
					{/if}
				</Button>
			</Dialog.Footer>
		</Dialog.Header>
	</Dialog.Content>
</Dialog.Root>
