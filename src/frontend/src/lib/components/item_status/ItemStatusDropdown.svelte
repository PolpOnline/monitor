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
	import { type ChangeVisibilityRequest, type SystemData, type Visibility } from '$lib/bindings';
	import { page } from '$app/stores';
	import { toast } from 'svelte-sonner';

	let className = '';

	// noinspection ReservedWordAsName
	export { className as class };

	export let data: SystemData;

	$: isPublic = $targetSystemData ? $targetSystemData.visibility === 'public' : undefined;

	let isVisibilityChanging = false;

	async function changeVisibility(newVisibility: Visibility, id: string) {
		isVisibilityChanging = true;
		const request = { id, visibility: newVisibility } as ChangeVisibilityRequest;

		const res = await fetch(`/api/change_visibility`, {
			method: 'PATCH',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(request)
		});

		if (res.ok) {
			toast.success('Successfully changed visibility to ' + newVisibility);
		} else {
			toast.error('Failed to change visibility: ' + (await res.text()));
		}

		invalidateAll();

		isVisibilityChanging = false;
	}
</script>

<DropdownMenu.Root onOpenChange={() => targetSystemData.set(data)}>
	<DropdownMenu.Trigger class={className}>
		<LucideEllipsis class="h-6 w-6" />
	</DropdownMenu.Trigger>
	<DropdownMenu.Content>
		<DropdownMenu.Group>
			<DropdownMenu.Item
				on:click={() => {
					if (!$targetSystemData) return;

					navigator.clipboard.writeText(`${API_URL}/ping_status/${$targetSystemData.id}`);

					toast.success('Copied endpoint URL');
				}}
			>
				<LucideClipboardCopy class="mr-2 h-4 w-4" />
				Copy endpoint URL
			</DropdownMenu.Item>
			<DropdownMenu.Item
				on:click={async () => {
					if (!$targetSystemData) return;

					const newVisibility = isPublic ? 'private' : 'public';

					$targetSystemData.visibility = newVisibility;

					await changeVisibility(newVisibility, $targetSystemData.id);
				}}
			>
				{#if isVisibilityChanging}
					<LineMdLoadingLoop class="mr-2 h-4 w-4" />
					Changing visibility...
				{:else if isPublic}
					<LucideLock class="mr-2 h-4 w-4" />
					Make Private
				{:else}
					<LucideEarth class="mr-2 h-4 w-4" />
					Make Public
				{/if}
			</DropdownMenu.Item>
			{#if isPublic}
				<DropdownMenu.Item
					on:click={() => {
						if (!$targetSystemData) return;

						navigator.clipboard.writeText(`${$page.url.origin}/public/${$targetSystemData.id}`);
					}}
				>
					<LucideLink class="mr-2 h-4 w-4" />
					Copy public link
				</DropdownMenu.Item>
			{/if}
			<DropdownMenu.Item
				on:click={() => {
					$presetDialogOpen = true;
				}}
			>
				<LucideSettings class="mr-2 h-4 w-4" />
				Configuration presets
			</DropdownMenu.Item>
			<DropdownMenu.Item
				on:click={() => {
					$editSystemNameDialogOpen = true;
				}}
			>
				<LucidePencilLine class="mr-2 h-4 w-4" />
				Edit name
			</DropdownMenu.Item>
			<DropdownMenu.Item
				class="text-red-600"
				data-sveltekit-preload-data="off"
				on:click={() => {
					$deleteSystemDialogOpen = true;
				}}
			>
				<LucideTrash2 class="mr-2 h-4 w-4" />
				Delete
			</DropdownMenu.Item>
		</DropdownMenu.Group>
	</DropdownMenu.Content>
</DropdownMenu.Root>
