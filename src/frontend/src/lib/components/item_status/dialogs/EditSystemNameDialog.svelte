<script lang="ts">
	// noinspection ES6UnusedImports
	import * as Dialog from '$components/ui/dialog';
	import { Button } from '$components/ui/form';
	import { editSystemNameDialogOpen, targetSystemData } from '$components/stores/popovers.store';
	import { invalidateAll } from '$app/navigation';
	import { Input } from '$components/ui/input';
	import LineMdLoadingLoop from '~icons/line-md/loading-loop';
	import { toast } from 'svelte-sonner';
	import { getTranslate, T } from '@tolgee/svelte';

	const { t } = getTranslate();

	let newSystemName = '';

	async function editSystemName() {
		isLoading = true;
		const id = $targetSystemData?.id;

		newSystemName = newSystemName.trim();

		const res = await fetch(`/api/edit_system_name`, {
			method: 'PATCH',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({ id, name: newSystemName })
		});

		if (res.ok) {
			toast.success($t('edit_system_name_dialog.success'));
		} else {
			toast.error($t('edit_system_name_dialog.fail', { error: await res.text() }));
		}

		invalidateAll();

		isLoading = false;
		editSystemNameDialogOpen.set(false);
	}

	let isLoading = false;
</script>

<Dialog.Root bind:open={$editSystemNameDialogOpen}>
	<Dialog.Trigger>
		<slot />
	</Dialog.Trigger>
	<Dialog.Content>
		<Dialog.Header>
			<Dialog.Title>
				<T keyName="edit_system_name_dialog.title" params={{ name: $targetSystemData?.name }} />
			</Dialog.Title>

			<Input
				bind:value={newSystemName}
				class="!my-3"
				placeholder={$t('edit_system_name_dialog.new_system_name')}
			/>

			<Dialog.Footer>
				<Button on:click={() => editSystemName()}>
					{#if !isLoading}
						<T keyName="save" />
					{:else}
						<LineMdLoadingLoop class="h-6 w-6" />
					{/if}
				</Button>
			</Dialog.Footer>
		</Dialog.Header>
	</Dialog.Content>
</Dialog.Root>
