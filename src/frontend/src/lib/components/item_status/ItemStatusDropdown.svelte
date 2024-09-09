<script lang="ts">
	// noinspection ES6UnusedImports
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import LucideEllipsis from '~icons/lucide/ellipsis';
	import LucideTrash2 from '~icons/lucide/trash-2';
	import {
		deleteSystemDialogOpen,
		deleteSystemDialogId
	} from '$lib/components/stores/popovers.store';

	let className = '';

	// noinspection ReservedWordAsName
	export { className as class };

	export let systemId: string;
</script>

<DropdownMenu.Root>
	<DropdownMenu.Trigger class={className}>
		<LucideEllipsis class="h-6 w-6" />
	</DropdownMenu.Trigger>
	<DropdownMenu.Content>
		<DropdownMenu.Group>
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
