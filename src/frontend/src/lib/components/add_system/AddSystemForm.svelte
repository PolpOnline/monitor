<script lang="ts">
	// noinspection ES6UnusedImports
	import * as Sheet from '$lib/components/ui/sheet/index.js';
	// noinspection ES6UnusedImports
	import { Button } from '$lib/components/ui/button/index.js';
	import { Input } from '$lib/components/ui/input/index.js';
	import DateTimePicker from '$components/date_time_picker/DateTimePicker.svelte';
	// noinspection ES6UnusedImports
	import * as Form from '$lib/components/ui/form';
	// noinspection ES6UnusedImports
	import * as Drawer from '$lib/components/ui/drawer';
	import { formSchema, type FormSchema } from './schema';
	import { type Infer, superForm, type SuperValidated } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import { addSystemSheetOpen } from '$lib/components/stores/popovers.store';
	import LineMdLoadingLoop from '~icons/line-md/loading-loop';

	export let data: SuperValidated<Infer<FormSchema>>;

	// noinspection JSUnusedGlobalSymbols
	const form = superForm(data, {
		validators: zodClient(formSchema),
		dataType: 'json',
		onUpdated: () => {
			$addSystemSheetOpen = false;
		},
		invalidateAll: true
	});

	const { form: formData, enhance } = form;

	export let delayed = form.delayed;

	let className = '';

	// noinspection ReservedWordAsName
	export { className as class };

	export let typeOfWrapper: 'sheet' | 'drawer' = 'sheet';
</script>

<form method="POST" action="?/add_system" use:enhance class={className}>
	<div class="p-4">
		<Form.Field {form} name="name">
			<Form.Control let:attrs>
				<Form.Label>Name</Form.Label>
				<Input {...attrs} bind:value={$formData.name} />
			</Form.Control>
			<Form.Description>Name of the system</Form.Description>
			<Form.FieldErrors />
		</Form.Field>

		<Form.Field {form} name="frequency">
			<Form.Control let:attrs>
				<Form.Label>Check frequency (in minutes)</Form.Label>
				<Input {...attrs} bind:value={$formData.frequency} type="number" />
			</Form.Control>
			<Form.Description>How often the system should be checked</Form.Description>
			<Form.FieldErrors />
		</Form.Field>

		<Form.Field {form} name="starts_at">
			<Form.Control let:attrs>
				<Form.Label>Starting date and time</Form.Label>
				<DateTimePicker {...attrs} bind:value={$formData.starts_at} />
			</Form.Control>
			<Form.Description>When the system should start being monitored</Form.Description>
			<Form.FieldErrors />
		</Form.Field>
	</div>

	{#if typeOfWrapper === 'sheet'}
		<Sheet.Footer>
			<Form.Button>
				{#if !$delayed}
					Add System
				{:else}
					<LineMdLoadingLoop class="h-6 w-6" />
				{/if}
			</Form.Button>
		</Sheet.Footer>
	{:else}
		<Drawer.Footer>
			<Form.Button>
				{#if !$delayed}
					Add System
				{:else}
					<LineMdLoadingLoop class="h-6 w-6" />
				{/if}
			</Form.Button>
		</Drawer.Footer>
	{/if}
</form>
