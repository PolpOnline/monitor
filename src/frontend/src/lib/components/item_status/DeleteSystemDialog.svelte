<script lang="ts">
	// noinspection ES6UnusedImports
	import * as Dialog from '$lib/components/ui/dialog';
	import { Button } from '$components/ui/form';
	import { deleteSystemDialogId, deleteSystemDialogOpen } from '$components/stores/popovers.store';
	import { invalidateAll } from '$app/navigation';

	async function deleteSystem() {
		const id = $deleteSystemDialogId;

		await fetch(`/api/delete_system`, {
			method: 'DELETE',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({ id })
		});

		invalidateAll();

		deleteSystemDialogOpen.set(false);
	}
</script>

<Dialog.Root bind:open={$deleteSystemDialogOpen}>
	<Dialog.Trigger>
		<slot />
	</Dialog.Trigger>
	<Dialog.Content>
		<Dialog.Header>
			<Dialog.Title>Are you absolutely sure?</Dialog.Title>
			<Dialog.Description>
				This action cannot be undone. This will permanently delete the system and all of its data.
			</Dialog.Description>

			<Dialog.Footer>
				<Button variant="secondary" on:click={() => ($deleteSystemDialogOpen = false)}>
					Cancel
				</Button>
				<Button variant="destructive" on:click={() => deleteSystem()}>Delete</Button>
			</Dialog.Footer>
		</Dialog.Header>
	</Dialog.Content>
</Dialog.Root>
