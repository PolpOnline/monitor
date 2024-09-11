<script lang="ts">
	import ItemStatus from '$components/item_status/ItemStatus.svelte';
	import type { PageData } from './$types';
	import AddSystem from '$components/add_system/AddSystem.svelte';
	import { title } from '$components/stores/title.store';
	import DeleteSystemDialog from '$components/item_status/DeleteSystemDialog.svelte';
	import EditSystemNameDialog from '$components/item_status/EditSystemNameDialog.svelte';
	import { fly, type FlyParams } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';
	import { flip, type AnimationConfig } from 'svelte/animate';

	title.set('Monitor');

	export let data: PageData;

	const inParams: FlyParams = { y: '100%', duration: 300, easing: cubicOut };
	const outParams: FlyParams = { x: '100%', duration: 700, easing: cubicOut };
	const animateParams: AnimationConfig = { delay: 700, duration: 300, easing: cubicOut };
</script>

<svelte:head>
	<title>Monitor</title>
</svelte:head>

<main>
	{#each data.systems as system (system.name)}
		<div class="my-3" in:fly={inParams} out:fly={outParams} animate:flip={animateParams}>
			<ItemStatus data={system} />
		</div>
	{/each}
</main>

<AddSystem data={data.form} />

<DeleteSystemDialog />

<EditSystemNameDialog />
