<script lang="ts">
	// noinspection ES6UnusedImports
	import * as Dialog from '$components/ui/dialog';
	import { Button } from '$components/ui/form';
	import { editSystemNameDialogOpen, targetSystemData } from '$components/stores/popovers.store';
	import { invalidateAll } from '$app/navigation';
	import { Input } from '$components/ui/input';
	import LineMdLoadingLoop from '~icons/line-md/loading-loop';

	let newSystemName = '';

	async function editSystemName() {
		isLoading = true;
		const id = $targetSystemData?.id;

		newSystemName = newSystemName.trim();

		await fetch(`/api/edit_system_name`, {
			method: 'PATCH',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({ id, name: newSystemName })
		});

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
			<Dialog.Title>Edit system name of {$targetSystemData?.name}</Dialog.Title>

			<Input bind:value={newSystemName} class="!my-3" placeholder="New system name" />

			<Dialog.Footer>
				<Button on:click={() => editSystemName()}>
					{#if !isLoading}
						Save
					{:else}
						<LineMdLoadingLoop class="h-6 w-6" />
					{/if}
				</Button>
			</Dialog.Footer>
		</Dialog.Header>
	</Dialog.Content>
</Dialog.Root>
