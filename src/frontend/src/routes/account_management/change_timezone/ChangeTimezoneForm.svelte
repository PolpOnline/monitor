<script lang="ts">
	// noinspection ES6UnusedImports
	import * as Form from '$components/ui/form';
	import { Input } from '$components/ui/input';
	import { formSchema as changeTimezoneFormSchema, type FormSchema } from './schema';
	import { type Infer, superForm, type SuperValidated } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import LineMdLoadingLoop from '~icons/line-md/loading-loop';

	export let data: SuperValidated<Infer<FormSchema>>;

	const form = superForm(data, {
		validators: zodClient(changeTimezoneFormSchema)
	});

	const { form: formData, enhance, message, delayed } = form;
</script>

<form action="?/change_timezone" method="POST" use:enhance>
	<Form.Field {form} name="timezone">
		<Form.Control let:attrs>
			<Form.Label>Timezone</Form.Label>
			<Input {...attrs} bind:value={$formData.timezone} />
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>
	{#if $message}
		<div class="text-red-600">{$message}</div>
	{/if}
	<Form.Button class="mt-8 w-full">
		{#if !$delayed}
			Change timezone
		{:else}
			<LineMdLoadingLoop class="h-6 w-6" />
		{/if}
	</Form.Button>
</form>
