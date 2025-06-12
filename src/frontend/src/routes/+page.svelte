<script lang="ts">
	import ItemStatus from '$components/item_status/ItemStatus.svelte';
	import type { PageData } from './$types';
	import AddSystem from '$components/add_system/AddSystem.svelte';
	import { title } from '$lib/stores/title.store';
	import { fly } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';
	import { flip } from 'svelte/animate';
	import PageSelector from '$components/PageSelector.svelte';

	title.set('Monitor');

	const { data }: { data: PageData } = $props();

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
					in:fly={{ y: '100%', duration: 300, easing: cubicOut }}
					out:fly={{ x: '100%', duration: 700, easing: cubicOut }}
					animate:flip={{ delay: 0, duration: 300, easing: cubicOut }}
					class="h-full last:odd:lg:col-span-2"
				>
					<ItemStatus data={system} />
				</div>
			{/each}
		</div>
	</div>

	<AddSystem data={form} />
</main>
