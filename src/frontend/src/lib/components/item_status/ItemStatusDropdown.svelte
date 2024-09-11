<script lang="ts">
	// noinspection ES6UnusedImports
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import LucideEllipsis from '~icons/lucide/ellipsis';
	import LucideTrash2 from '~icons/lucide/trash-2';
	import {
		deleteSystemDialogId,
		deleteSystemDialogOpen,
		editSystemNameDialogId,
		editSystemNameDialogOldName,
		editSystemNameDialogOpen
	} from '$lib/components/stores/popovers.store';
	import LucideClipboardCopy from '~icons/lucide/clipboard-copy';
	import LucidePencilLine from '~icons/lucide/pencil-line';
	import { API_URL } from '$lib/api/api';

	let className = '';

	// noinspection ReservedWordAsName
	export { className as class };

	export let systemId: string;
	export let systemName: string;
</script>

<DropdownMenu.Root>
	<DropdownMenu.Trigger class={className}>
		<LucideEllipsis class="h-6 w-6" />
	</DropdownMenu.Trigger>
	<DropdownMenu.Content>
		<DropdownMenu.Group>
			<DropdownMenu.Item
				on:click={() => {
					navigator.clipboard.writeText(`${API_URL}/ping_status/${systemId}`);
				}}
			>
				<LucideClipboardCopy class="mr-2 h-4 w-4" />
				Copy endpoint URL
			</DropdownMenu.Item>
			<DropdownMenu.Item
				on:click={() => {
					editSystemNameDialogId.set(systemId);
					editSystemNameDialogOldName.set(systemName);
					$editSystemNameDialogOpen = true;
				}}
			>
				<LucidePencilLine class="mr-2 h-4 w-4" />
				Edit name
			</DropdownMenu.Item>
			<DropdownMenu.Item
				data-sveltekit-preload-data="off"
				class="text-red-600"
				on:click={() => {
					$deleteSystemDialogOpen = true;
					deleteSystemDialogId.set(systemId);
				}}
			>
				<LucideTrash2 class="mr-2 h-4 w-4" />
				Delete
			</DropdownMenu.Item>
		</DropdownMenu.Group>
	</DropdownMenu.Content>
</DropdownMenu.Root>
