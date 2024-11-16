<script lang="ts">
	// noinspection ES6UnusedImports
	import * as Sheet from '$lib/components/ui/sheet/index.js';
	import { buttonVariants } from '$lib/components/ui/button/index.js';
	import HeroiconsPlus20Solid from '~icons/heroicons/plus-20-solid';
	import { type FormSchema } from './schema';
	import { type Infer, type SuperValidated } from 'sveltekit-superforms';
	import { addSystemSheetOpen } from '$lib/components/stores/popovers.store';
	import AddSystemForm from '$components/add_system/AddSystemForm.svelte';
	import Device from 'svelte-device-info';
	// noinspection ES6UnusedImports
	import * as Drawer from '$lib/components/ui/drawer';
	import { browser } from '$app/environment';
	import { getTranslate } from '@tolgee/svelte';
	import { cn } from '$lib/utils';

	const { t } = getTranslate();

	export let data: SuperValidated<Infer<FormSchema>>;

	let isMobile = false;

	if (browser) {
		isMobile = Device.isMobile;
	}

	const title = $t('add_system.add_system');
	const description = $t('add_system.add_system_description');
</script>

{#if !isMobile}
	<Sheet.Root bind:open={$addSystemSheetOpen}>
		<Sheet.Trigger
			class={cn(buttonVariants({ variant: 'default' }), 'fixed bottom-5 right-5 h-12 w-12')}
			aria-label="Add device"
		>
			<HeroiconsPlus20Solid class="h-6 w-6" />
		</Sheet.Trigger>
		<Sheet.Content side="right">
			<Sheet.Header class="mb-3">
				<Sheet.Title>{title}</Sheet.Title>
				<Sheet.Description>
					{description}
				</Sheet.Description>
			</Sheet.Header>

			<AddSystemForm {data} typeOfWrapper="sheet" />
		</Sheet.Content>
	</Sheet.Root>
{:else}
	<Drawer.Root bind:open={$addSystemSheetOpen}>
		<Drawer.Trigger
			class={cn(buttonVariants({ variant: 'outline' }), 'fixed bottom-5 right-5 h-12 w-12')}
		>
			<HeroiconsPlus20Solid class="h-6 w-6" />
		</Drawer.Trigger>
		<Drawer.Content>
			<Drawer.Header>
				<Drawer.Title>{title}</Drawer.Title>
				<Drawer.Description>
					{description}
				</Drawer.Description>
			</Drawer.Header>

			<AddSystemForm {data} class="mx-5" typeOfWrapper="drawer" />
		</Drawer.Content>
	</Drawer.Root>
{/if}
