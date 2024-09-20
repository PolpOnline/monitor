<script lang="ts">
	import ItemStatus from '$components/item_status/ItemStatus.svelte';
	import type { PageData } from './$types';
	import { title } from '$components/stores/title.store';

	title.set('Public Monitor');

	export let data: PageData;

	title.set(data.system.name);
</script>

<ItemStatus data={data.system} showDropdown={false} />
