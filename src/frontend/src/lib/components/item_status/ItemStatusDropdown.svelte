<script lang="ts">
	// noinspection ES6UnusedImports
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import LucideEllipsis from '~icons/lucide/ellipsis';
	import LucideTrash2 from '~icons/lucide/trash-2';
	import LucideClipboardCopy from '~icons/lucide/clipboard-copy';
	import LucidePencilLine from '~icons/lucide/pencil-line';
	import { PUBLIC_API_URL } from '$lib/api/public-api';
	import LucideSettings from '~icons/lucide/settings';
	import LucideEarth from '~icons/lucide/earth';
	import LucideLock from '~icons/lucide/lock';
	import { invalidateAll } from '$app/navigation';
	import LineMdLoadingLoop from '~icons/line-md/loading-loop';
	import LucideLink from '~icons/lucide/link';
	import { page } from '$app/state';
	import { toast } from 'svelte-sonner';
	import { getTranslate, T } from '@tolgee/svelte';
	import type { components } from '$lib/api/schema';
	import PresetDialog from '$components/item_status/dialogs/PresetDialog.svelte';
	import EditSystemNameDialog from '$components/item_status/dialogs/EditSystemNameDialog.svelte';
	import DeleteSystemDialog from '$components/item_status/dialogs/DeleteSystemDialog.svelte';

	const { t } = getTranslate();

	const className = $state('');

	// noinspection ReservedWordAsName
	export { className as class };

	const { data }: { data: components['schemas']['SystemData'] } = $props();

	const isPublic = $derived(data ? data.visibility === 'public' : undefined);

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

		await invalidateAll();

		isVisibilityChanging = false;
	}
</script>

<DropdownMenu.Root>
	<DropdownMenu.Trigger
		class={className}
		aria-label={$t('item_status.options_for', { name: data.name })}
	>
		<LucideEllipsis class="size-6 cursor-pointer" />
	</DropdownMenu.Trigger>
	<DropdownMenu.Content>
		<DropdownMenu.Group>
			<DropdownMenu.Item
				onclick={() => {
					if (!data) return;

					navigator.clipboard.writeText(`${PUBLIC_API_URL}/ping_status/${data.id}`);

					toast.success($t('copied_endpoint_url'));
				}}
			>
				<LucideClipboardCopy class="mr-2 size-4" />
				<T keyName="copy_endpoint_url" />
			</DropdownMenu.Item>
			<DropdownMenu.Item
				onclick={async () => {
					if (!data) return;

					const newVisibility = isPublic ? 'private' : 'public';

					data.visibility = newVisibility;

					await changeVisibility(newVisibility, data.id);
				}}
			>
				{#if isVisibilityChanging}
					<LineMdLoadingLoop class="mr-2 size-4" />
					<T keyName="change_visibility.changing" />
				{:else if isPublic}
					<LucideLock class="mr-2 size-4" />
					<T keyName="change_visibility.make_private" />
				{:else}
					<LucideEarth class="mr-2 size-4" />
					<T keyName="change_visibility.make_public" />
				{/if}
			</DropdownMenu.Item>
			{#if isPublic}
				<DropdownMenu.Item
					onclick={() => {
						if (!data) return;

						navigator.clipboard.writeText(`${page.url.origin}/public/${data.id}`);
					}}
				>
					<LucideLink class="mr-2 size-4" />
					<T keyName="copy_public_link" />
				</DropdownMenu.Item>
			{/if}
			<PresetDialog targetSystemData={data}>
				<DropdownMenu.Item closeOnSelect={false}>
					<LucideSettings class="mr-2 size-4" />
					<T keyName="configuration_presets" />
				</DropdownMenu.Item>
			</PresetDialog>
			<EditSystemNameDialog targetSystemData={data}>
				<DropdownMenu.Item closeOnSelect={false}>
					<LucidePencilLine class="mr-2 size-4" />
					<T keyName="edit_name" />
				</DropdownMenu.Item>
			</EditSystemNameDialog>
			<DeleteSystemDialog targetSystemData={data}>
				<DropdownMenu.Item closeOnSelect={false} variant="destructive">
					<LucideTrash2 class="mr-2 size-4" />
					<T keyName="delete" />
				</DropdownMenu.Item>
			</DeleteSystemDialog>
		</DropdownMenu.Group>
	</DropdownMenu.Content>
</DropdownMenu.Root>
