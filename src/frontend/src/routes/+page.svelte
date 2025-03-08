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

	let { data }: { data: PageData } = $props();

	const inParams: FlyParams = { y: '100%', duration: 300, easing: cubicOut };
	const outParams: FlyParams = { x: '100%', duration: 700, easing: cubicOut };
	const animateParams: AnimationConfig = { delay: 0, duration: 300, easing: cubicOut };

	const systems = $derived(data.systems!);
	const form = $derived(data.form!);
</script>

<svelte:head>
	<title>Monitor</title>
</svelte:head>

<main>
	<div class="mx-4" data-vaul-drawer-wrapper>
		<PageSelector />

		<div class="grid grid-cols-1 gap-3 pb-20 lg:grid-cols-2">
			{#each systems as system (system.id)}
				<div
					in:fly={inParams}
					out:fly={outParams}
					animate:flip={animateParams}
					class="h-full last:odd:lg:col-span-2"
				>
					<ItemStatus data={system} />
				</div>
			{/each}
		</div>
	</div>

	<AddSystem data={form} />

	<DeleteSystemDialog />

	<EditSystemNameDialog />

	<PresetDialog />
</main>
