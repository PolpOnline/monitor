<script lang="ts">
	import ItemStatus from '$components/item_status/ItemStatus.svelte';
	import type { PageData } from './$types';
	import { title } from '$lib/stores/title.store';
	import PageSelector from '$components/PageSelector.svelte';

	title.set('Public Monitor');

	let { data }: { data: PageData } = $props();

	const system = $derived(data.system!);

	title.set(data.system!.name + ' status');
</script>

<div class="mx-4">
	<PageSelector />

	<ItemStatus data={system} showDropdown={false} />
</div>
