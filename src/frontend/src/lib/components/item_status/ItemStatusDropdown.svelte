<script lang="ts">
	// noinspection ES6UnusedImports
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import LucideEllipsis from '~icons/lucide/ellipsis';
	import LucideTrash2 from '~icons/lucide/trash-2';
	import {
		deleteSystemDialogOpen,
		editSystemNameDialogOpen,
		presetDialogOpen,
		targetSystemData
	} from '$lib/components/stores/popovers.store';
	import LucideClipboardCopy from '~icons/lucide/clipboard-copy';
	import LucidePencilLine from '~icons/lucide/pencil-line';
	import { API_URL } from '$lib/api/api';
	import LucideSettings from '~icons/lucide/settings';
	import LucideEarth from '~icons/lucide/earth';
	import LucideLock from '~icons/lucide/lock';
	import { invalidateAll } from '$app/navigation';
	import LineMdLoadingLoop from '~icons/line-md/loading-loop';
	import LucideLink from '~icons/lucide/link';
	import { page } from '$app/stores';
	import { toast } from 'svelte-sonner';
	import { getTranslate, T } from '@tolgee/svelte';
	import type { components } from '$lib/api/schema';

	const { t } = getTranslate();

	const className = $state('');

	// noinspection ReservedWordAsName
	export { className as class };

	const { data }: { data: components['schemas']['SystemData'] } = $props();

	const isPublic = $derived(
		$targetSystemData ? $targetSystemData.visibility === 'public' : undefined
	);

	let isVisibilityChanging = $state(false);

	async function changeVisibility(newVisibility: components['schemas']['Visibility'], id: string) {
		isVisibilityChanging = true;
		const request = {
			id,
			visibility: newVisibility
		} as components['schemas']['ChangeVisibilityRequest'];

		const res = await fetch(`/api/change_visibility`, {
			method: 'PATCH',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(request)
		});

		if (res.ok) {
			toast.success($t('change_visibility.success', { visibility: newVisibility }));
		} else {
			toast.error($t('change_visibility.failed', { error: await res.text() }));
		}

		invalidateAll();

		isVisibilityChanging = false;
	}
</script>

<DropdownMenu.Root onOpenChange={() => targetSystemData.set(data)}>
	<DropdownMenu.Trigger
		class={className}
		aria-label={$t('item_status.options_for', { name: data.name })}
	>
		<LucideEllipsis class="h-6 w-6" />
	</DropdownMenu.Trigger>
	<DropdownMenu.Content>
		<DropdownMenu.Group>
			<DropdownMenu.Item
				onclick={() => {
					if (!$targetSystemData) return;

					navigator.clipboard.writeText(`${API_URL}/ping_status/${$targetSystemData.id}`);

					toast.success($t('copied_endpoint_url'));
				}}
			>
				<LucideClipboardCopy class="mr-2 h-4 w-4" />
				<T keyName="copy_endpoint_url" />
			</DropdownMenu.Item>
			<DropdownMenu.Item
				onclick={async () => {
					if (!$targetSystemData) return;

					const newVisibility = isPublic ? 'private' : 'public';

					$targetSystemData.visibility = newVisibility;

					await changeVisibility(newVisibility, $targetSystemData.id);
				}}
			>
				{#if isVisibilityChanging}
					<LineMdLoadingLoop class="mr-2 h-4 w-4" />
					<T keyName="change_visibility.changing" />
				{:else if isPublic}
					<LucideLock class="mr-2 h-4 w-4" />
					<T keyName="change_visibility.make_private" />
				{:else}
					<LucideEarth class="mr-2 h-4 w-4" />
					<T keyName="change_visibility.make_public" />
				{/if}
			</DropdownMenu.Item>
			{#if isPublic}
				<DropdownMenu.Item
					onclick={() => {
						if (!$targetSystemData) return;

						navigator.clipboard.writeText(`${$page.url.origin}/public/${$targetSystemData.id}`);
					}}
				>
					<LucideLink class="mr-2 h-4 w-4" />
					<T keyName="copy_public_link" />
				</DropdownMenu.Item>
			{/if}
			<DropdownMenu.Item
				onclick={() => {
					$presetDialogOpen = true;
				}}
			>
				<LucideSettings class="mr-2 h-4 w-4" />
				<T keyName="configuration_presets" />
			</DropdownMenu.Item>
			<DropdownMenu.Item
				onclick={() => {
					$editSystemNameDialogOpen = true;
				}}
			>
				<LucidePencilLine class="mr-2 h-4 w-4" />
				<T keyName="edit_name" />
			</DropdownMenu.Item>
			<DropdownMenu.Item
				class="text-red-600"
				data-sveltekit-preload-data="off"
				onclick={() => {
					$deleteSystemDialogOpen = true;
				}}
			>
				<LucideTrash2 class="mr-2 h-4 w-4" />
				<T keyName="delete" />
			</DropdownMenu.Item>
		</DropdownMenu.Group>
	</DropdownMenu.Content>
</DropdownMenu.Root>
