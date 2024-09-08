<script lang="ts">
	// noinspection ES6UnusedImports
	import * as Form from '$lib/components/ui/form';
	import { Input } from '$lib/components/ui/input';
	import { formSchema, type FormSchema } from './schema';
	import { type Infer, superForm, type SuperValidated } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import LineMdLoadingLoop from '~icons/line-md/loading-loop';

	export let data: SuperValidated<Infer<FormSchema>>;

	const form = superForm(data, {
		validators: zodClient(formSchema)
	});

	const { form: formData, enhance, message, delayed } = form;
</script>

<form method="POST" use:enhance action="?/change_password">
	<Form.Field {form} name="old_password">
		<Form.Control let:attrs>
			<Form.Label>Old Password</Form.Label>
			<Input type="password" {...attrs} bind:value={$formData.old_password} />
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>
	<Form.Field {form} name="new_password">
		<Form.Control let:attrs>
			<Form.Label>New password</Form.Label>
			<Input type="password" {...attrs} bind:value={$formData.new_password} />
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>
	<Form.Field {form} name="new_password_confirm">
		<Form.Control let:attrs>
			<Form.Label>Confirm new password</Form.Label>
			<Input type="password" {...attrs} bind:value={$formData.new_password_confirm} />
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>
	{#if $message}
		<div class="text-red-600">{$message}</div>
	{/if}
	<Form.Button class="mt-8 w-full">
		{#if !$delayed}
			Submit
		{:else}
			<LineMdLoadingLoop class="h-6 w-6" />
		{/if}
	</Form.Button>
</form>
