<script lang="ts">
	// noinspection ES6UnusedImports
	import * as Sheet from '$lib/components/ui/sheet/index.js';
	import { Button } from '$lib/components/ui/button/index.js';
	import HeroiconsPlus20Solid from '~icons/heroicons/plus-20-solid';
	import { type FormSchema } from './schema';
	import { type Infer, type SuperValidated } from 'sveltekit-superforms';
	import { addSystemSheetOpen } from '$lib/components/stores/popovers.store';
	import AddSystemForm from '$components/add_system/AddSystemForm.svelte';
	import Device from 'svelte-device-info';
	// noinspection ES6UnusedImports
	import * as Drawer from '$lib/components/ui/drawer';

	export let data: SuperValidated<Infer<FormSchema>>;

	const isMobile = Device.isMobile;

	const title = 'Add a system';
	const description = 'Add a new system to the monitor by filling out the form below.';
</script>

{#if !isMobile}
	<Sheet.Root bind:open={$addSystemSheetOpen}>
		<Sheet.Trigger asChild let:builder>
			<Button builders={[builder]} class="fixed bottom-5 right-5 h-12 w-12">
				<HeroiconsPlus20Solid class="h-6 w-6" />
			</Button>
		</Sheet.Trigger>
		<Sheet.Content side="right">
			<Sheet.Header class="mb-3">
				<Sheet.Title>{title}</Sheet.Title>
				<Sheet.Description>
					{description}
				</Sheet.Description>
			</Sheet.Header>

			<AddSystemForm {data} typeOfWrapper="sheet">
				<slot name="footer">
					<Sheet.Footer />
				</slot>
			</AddSystemForm>
		</Sheet.Content>
	</Sheet.Root>
{:else}
	<Drawer.Root bind:open={$addSystemSheetOpen}>
		<Drawer.Trigger asChild let:builder>
			<Button
				builders={[builder]}
				class="fixed bottom-5 right-5 h-12 w-12 {$addSystemSheetOpen ? 'hidden' : ''}"
			>
				<HeroiconsPlus20Solid class="h-6 w-6" />
			</Button>
		</Drawer.Trigger>
		<Drawer.Content>
			<Drawer.Header>
				<Drawer.Title>{title}</Drawer.Title>
				<Drawer.Description>
					{description}
				</Drawer.Description>
			</Drawer.Header>

			<AddSystemForm {data} class="mx-5" typeOfWrapper="drawer">
				<slot name="footer">
					<Drawer.Footer />
				</slot>
			</AddSystemForm>
		</Drawer.Content>
	</Drawer.Root>
{/if}
