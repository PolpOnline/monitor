<script lang="ts">
	import ItemStatus from '$components/item_status/ItemStatus.svelte';
	import type { PageData } from './$types';
	import { title } from '$components/stores/title.store';
	import PageSelector from '$components/PageSelector.svelte';

	title.set('Public Monitor');

	let { data }: { data: PageData } = $props();

	title.set(data.system.name + ' status');
</script>

<div class="mx-4">
	<PageSelector />

	<ItemStatus data={data.system} showDropdown={false} />
</div>
