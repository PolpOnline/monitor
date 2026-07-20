<script lang="ts">
	// noinspection ES6UnusedImports
	import * as Sheet from '$lib/components/ui/sheet/index.js';
	import { buttonVariants } from '$lib/components/ui/button/index.js';
	import HeroiconsPlus20Solid from '~icons/heroicons/plus-20-solid';
	import { type FormSchema } from './schema';
	import { type Infer, type SuperValidated } from 'sveltekit-superforms';
	import AddSystemForm from '$components/add_system/AddSystemForm.svelte';
	import { getTranslate } from '@tolgee/svelte';
	import { cn } from '$lib/utils';

	let open = $state(false);
	const { t } = getTranslate();

	const {
		data
	}: {
		data: SuperValidated<Infer<FormSchema>>;
	} = $props();

	const title = $t('add_system.add_system');
	const description = $t('add_system.add_system_description');
</script>

<Sheet.Root bind:open>
	<Sheet.Trigger
		class={cn(buttonVariants({ variant: 'default' }), 'fixed right-5 bottom-5 z-50 size-12')}
		aria-label="Add device"
	>
		<HeroiconsPlus20Solid class="size-6" />
	</Sheet.Trigger>
	<Sheet.Content
		side="right"
		class="data-open:animate-in data-open:fade-in-0 data-open:slide-in-from-right-10 data-closed:animate-out data-closed:fade-out-0 data-closed:slide-out-to-right-10"
	>
		<Sheet.Header>
			<Sheet.Title>{title}</Sheet.Title>
			<Sheet.Description>
				{description}
			</Sheet.Description>
		</Sheet.Header>

		<AddSystemForm {data} typeOfWrapper="sheet" />
	</Sheet.Content>
</Sheet.Root>
