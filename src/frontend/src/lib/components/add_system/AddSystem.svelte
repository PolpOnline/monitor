<script lang="ts">
	// noinspection ES6UnusedImports
	import * as Sheet from '$lib/components/ui/sheet/index.js';
	import { buttonVariants } from '$lib/components/ui/button/index.js';
	import HeroiconsPlus20Solid from '~icons/heroicons/plus-20-solid';
	import { type FormSchema } from './schema';
	import { type Infer, type SuperValidated } from 'sveltekit-superforms';
	import AddSystemForm from '$components/add_system/AddSystemForm.svelte';
	import Device from 'svelte-device-info';
	// noinspection ES6UnusedImports
	import * as Drawer from '$lib/components/ui/drawer';
	import { browser } from '$app/environment';
	import { getTranslate } from '@tolgee/svelte';
	import { cn } from '$lib/utils';

	let open = $state(false);
	const { t } = getTranslate();

	const {
		data
	}: {
		data: SuperValidated<Infer<FormSchema>>;
	} = $props();

	let isMobile = $state(true);

	if (browser) {
		isMobile = Device.isMobile;
	}

	const title = $t('add_system.add_system');
	const description = $t('add_system.add_system_description');
</script>

{#if !isMobile}
	<Sheet.Root bind:open>
		<Sheet.Trigger
			class={cn(buttonVariants({ variant: 'default' }), 'fixed right-5 bottom-5 size-12')}
			aria-label="Add device"
		>
			<HeroiconsPlus20Solid class="size-6" />
		</Sheet.Trigger>
		<Sheet.Content side="right">
			<Sheet.Header>
				<Sheet.Title>{title}</Sheet.Title>
				<Sheet.Description>
					{description}
				</Sheet.Description>
			</Sheet.Header>

			<AddSystemForm {data} typeOfWrapper="sheet" bind:open />
		</Sheet.Content>
	</Sheet.Root>
{:else}
	<Drawer.Root bind:open>
		<Drawer.Trigger
			class={cn(buttonVariants({ variant: 'default' }), 'fixed right-5 bottom-5 size-12')}
		>
			<HeroiconsPlus20Solid class="size-6" />
		</Drawer.Trigger>
		<Drawer.Content>
			<Drawer.Header>
				<Drawer.Title>{title}</Drawer.Title>
				<Drawer.Description>
					{description}
				</Drawer.Description>
			</Drawer.Header>

			<AddSystemForm {data} class="mx-5" typeOfWrapper="drawer" bind:open />
		</Drawer.Content>
	</Drawer.Root>
{/if}
