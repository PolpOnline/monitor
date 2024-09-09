<script lang="ts">
	// noinspection ES6UnusedImports
	import * as Sheet from '$lib/components/ui/sheet/index.js';
	import { Button } from '$lib/components/ui/button/index.js';
	import { Input } from '$lib/components/ui/input/index.js';
	import HeroiconsPlus20Solid from '~icons/heroicons/plus-20-solid';
	import LineMdLoadingLoop from '~icons/line-md/loading-loop';
	import DateTimePicker from '$components/date_time_picker/DateTimePicker.svelte';
	// noinspection ES6UnusedImports
	import * as Form from '$lib/components/ui/form';
	import { formSchema, type FormSchema } from './schema';
	import SuperDebug, { type Infer, superForm, type SuperValidated } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';

	export let data: SuperValidated<Infer<FormSchema>>;

	// noinspection JSUnusedGlobalSymbols
	const form = superForm(data, {
		validators: zodClient(formSchema),
		dataType: 'json',
		onUpdated: () => {
			sheetOpen = false;
		},
		invalidateAll: true
	});

	const { form: formData, enhance, delayed } = form;

	let sheetOpen = false;
</script>

<Sheet.Root bind:open={sheetOpen}>
	<Sheet.Trigger asChild let:builder>
		<Button builders={[builder]} class="fixed bottom-7 right-7 h-12 w-12">
			<HeroiconsPlus20Solid class="h-6 w-6" />
		</Button>
	</Sheet.Trigger>
	<Sheet.Content side="right">
		<form method="POST" action="?/add_system" use:enhance>
			<Sheet.Header class="mb-3">
				<Sheet.Title>Add a system</Sheet.Title>
				<Sheet.Description>
					Add a new system to the monitor by filling out the form below.
				</Sheet.Description>
			</Sheet.Header>

			<!-- Form part -->
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

			<Sheet.Footer>
				<Form.Button>
					{#if !$delayed}
						Add System
					{:else}
						<LineMdLoadingLoop class="h-6 w-6" />
					{/if}
				</Form.Button>
			</Sheet.Footer>
		</form>
	</Sheet.Content>
</Sheet.Root>
