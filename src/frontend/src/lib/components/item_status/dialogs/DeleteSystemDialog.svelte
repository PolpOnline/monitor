<script lang="ts">
	// noinspection ES6UnusedImports
	import * as Dialog from '$components/ui/dialog';
	import { Button } from '$components/ui/form';
	import { deleteSystemDialogOpen, targetSystemData } from '$components/stores/popovers.store';
	import { invalidateAll } from '$app/navigation';
	import LineMdLoadingLoop from '~icons/line-md/loading-loop';
	import { toast } from 'svelte-sonner';
	import { getTranslate, T } from '@tolgee/svelte';

	const { t } = getTranslate();

	async function deleteSystem() {
		isLoading = true;
		const id = $targetSystemData?.id;

		const res = await fetch(`/api/delete_system`, {
			method: 'DELETE',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({ id })
		});

		if (res.ok) {
			toast.success($t('delete_system_dialog.success'));
		} else {
			toast.error($t('delete_system_dialog.fail', { error: await res.text() }));
		}

		invalidateAll();

		isLoading = false;
		deleteSystemDialogOpen.set(false);
	}

	let isLoading = false;
</script>

<Dialog.Root bind:open={$deleteSystemDialogOpen}>
	<Dialog.Trigger>
		<slot />
	</Dialog.Trigger>
	<Dialog.Content>
		<Dialog.Header>
			<Dialog.Title>
				<T keyName="delete_system_dialog.title" />
			</Dialog.Title>
			<Dialog.Description>
				<T keyName="delete_system_dialog.description" />
			</Dialog.Description>

			<Dialog.Footer>
				<Button
					class="mt-3 sm:mt-0"
					on:click={() => ($deleteSystemDialogOpen = false)}
					variant="secondary"
				>
					<T keyName="cancel" />
				</Button>
				<Button class="mt-5 sm:mt-0" on:click={() => deleteSystem()} variant="destructive">
					{#if !isLoading}
						<T keyName="delete" />
					{:else}
						<LineMdLoadingLoop class="h-6 w-6" />
					{/if}
				</Button>
			</Dialog.Footer>
		</Dialog.Header>
	</Dialog.Content>
</Dialog.Root>
