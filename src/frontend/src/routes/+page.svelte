<script lang="ts">
	import ItemStatus from '$components/item_status/ItemStatus.svelte';
	import type { PageData } from './$types';
	import AddSystem from '$components/add_system/AddSystem.svelte';
	import { title } from '$components/stores/title.store';
	import DeleteSystemDialog from '$components/item_status/DeleteSystemDialog.svelte';
	import EditSystemNameDialog from '$components/item_status/EditSystemNameDialog.svelte';

	title.set('Monitor');

	export let data: PageData;
</script>

<svelte:head>
	<title>Monitor</title>
</svelte:head>

<main>
	{#each data.systems as system (system.name)}
		<div class="my-3">
			<ItemStatus data={system} />
		</div>
	{/each}
</main>

<AddSystem data={data.form} />

<DeleteSystemDialog />

<EditSystemNameDialog />
