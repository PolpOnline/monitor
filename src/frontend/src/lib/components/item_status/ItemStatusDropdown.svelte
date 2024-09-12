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
	import type { SystemData } from '../../../../../backend/bindings';

	let className = '';

	// noinspection ReservedWordAsName
	export { className as class };

	export let data: SystemData;

	$targetSystemData = data;
</script>

<DropdownMenu.Root>
	<DropdownMenu.Trigger class={className}>
		<LucideEllipsis class="h-6 w-6" />
	</DropdownMenu.Trigger>
	<DropdownMenu.Content>
		<DropdownMenu.Group>
			<DropdownMenu.Item
				on:click={() => {
					navigator.clipboard.writeText(`${API_URL}/ping_status/${data.id}`);
				}}
			>
				<LucideClipboardCopy class="mr-2 h-4 w-4" />
				Copy endpoint URL
			</DropdownMenu.Item>
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
