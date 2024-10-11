<script lang="ts">
	// noinspection ES6UnusedImports
	import * as Dialog from '$components/ui/dialog';
	import { Button } from '$components/ui/form';
	import { deleteSystemDialogOpen, targetSystemData } from '$components/stores/popovers.store';
	import { invalidateAll } from '$app/navigation';
	import LineMdLoadingLoop from '~icons/line-md/loading-loop';
	import { toast } from 'svelte-sonner';

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
			toast.success('System deleted successfully');
		} else {
			toast.error('Failed to delete the system: ' + (await res.text()));
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
			<Dialog.Title>Are you absolutely sure?</Dialog.Title>
			<Dialog.Description>
				This action cannot be undone. This will permanently delete the system and all of its data.
			</Dialog.Description>

			<Dialog.Footer>
				<Button
					class="mt-3 sm:mt-0"
					on:click={() => ($deleteSystemDialogOpen = false)}
					variant="secondary"
				>
					Cancel
				</Button>
				<Button class="mt-5 sm:mt-0" on:click={() => deleteSystem()} variant="destructive">
					{#if !isLoading}
						Delete
					{:else}
						<LineMdLoadingLoop class="h-6 w-6" />
					{/if}
				</Button>
			</Dialog.Footer>
		</Dialog.Header>
	</Dialog.Content>
</Dialog.Root>
