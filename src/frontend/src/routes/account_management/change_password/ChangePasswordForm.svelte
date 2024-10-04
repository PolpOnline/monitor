<script lang="ts">
	// noinspection ES6UnusedImports
	import * as Form from '$components/ui/form';
	import { Input } from '$components/ui/input';
	import { formSchema as changePasswordFormSchema, type FormSchema } from './schema';
	import { type Infer, superForm, type SuperValidated } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import LineMdLoadingLoop from '~icons/line-md/loading-loop';

	export let data: SuperValidated<Infer<FormSchema>>;

	const form = superForm(data, {
		validators: zodClient(changePasswordFormSchema)
	});

	const { form: formData, enhance, message, delayed } = form;
</script>

<form action="?/change_password" method="POST" use:enhance>
	<Form.Field {form} name="old_password">
		<Form.Control let:attrs>
			<Form.Label>Old Password</Form.Label>
			<Input {...attrs} bind:value={$formData.old_password} type="password" />
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>
	<Form.Field {form} name="new_password">
		<Form.Control let:attrs>
			<Form.Label>New password</Form.Label>
			<Input {...attrs} bind:value={$formData.new_password} type="password" />
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>
	<Form.Field {form} name="new_password_confirm">
		<Form.Control let:attrs>
			<Form.Label>Confirm new password</Form.Label>
			<Input {...attrs} bind:value={$formData.new_password_confirm} type="password" />
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>
	{#if $message}
		<div class="text-red-600">{$message}</div>
	{/if}
	<Form.Button class="mt-8 w-full">
		{#if !$delayed}
			Change password
		{:else}
			<LineMdLoadingLoop class="h-6 w-6" />
		{/if}
	</Form.Button>
</form>
