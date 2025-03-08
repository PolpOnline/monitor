<script lang="ts">
	// noinspection ES6UnusedImports
	import * as Dialog from '$components/ui/dialog';
	import { Button } from '$components/ui/form';
	import { invalidateAll } from '$app/navigation';
	import LineMdLoadingLoop from '~icons/line-md/loading-loop';
	import { toast } from 'svelte-sonner';
	import { getTranslate, T } from '@tolgee/svelte';
	import type { components } from '$lib/api/schema';
	import type { Snippet } from 'svelte';

	let {
		children,
		targetSystemData
	}: {
		children: Snippet;
		targetSystemData: components['schemas']['SystemData'];
	} = $props();

	const { t } = getTranslate();

	let open = $state(false);

	async function deleteSystem() {
		isLoading = true;
		const id = targetSystemData?.id;

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
			<Dialog.Title>
				<T keyName="delete_system_dialog.title" />
			</Dialog.Title>
			<Dialog.Description>
				<T keyName="delete_system_dialog.description" />
			</Dialog.Description>

			<Dialog.Footer>
				<Button class="mt-3 sm:mt-0" onclick={() => (open = false)} variant="secondary">
					<T keyName="cancel" />
				</Button>
				<Button class="mt-5 sm:mt-0" onclick={() => deleteSystem()} variant="destructive">
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
