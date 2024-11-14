<script lang="ts">
	import ItemStatus from '$components/item_status/ItemStatus.svelte';
	import type { PageData } from './$types';
	import AddSystem from '$components/add_system/AddSystem.svelte';
	import { title } from '$components/stores/title.store';
	import DeleteSystemDialog from '$components/item_status/dialogs/DeleteSystemDialog.svelte';
	import EditSystemNameDialog from '$components/item_status/dialogs/EditSystemNameDialog.svelte';
	import PresetDialog from '$components/item_status/dialogs/PresetDialog.svelte';
	import { fly, type FlyParams } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';
	import { type AnimationConfig, flip } from 'svelte/animate';
	import PageSelector from '$components/PageSelector.svelte';

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
	<div class="mx-4" data-vaul-drawer-wrapper>
		<PageSelector class="mb-3" />

		<div class="grid grid-cols-1 gap-3 lg:grid-cols-2">
			{#each data.systems as system (system.id)}
				<div
					in:fly={inParams}
					out:fly={outParams}
					animate:flip={animateParams}
					class:custom-col-width={data.systems.length % 2 === 1}
					class="h-full"
				>
					<ItemStatus data={system} />
				</div>
			{/each}
		</div>
	</div>

	<AddSystem data={data.form} />

	<DeleteSystemDialog />

	<EditSystemNameDialog />

	<PresetDialog />
</main>

<style lang="postcss">
	.custom-col-width:last-of-type {
		@apply lg:col-span-2;
	}
</style>
