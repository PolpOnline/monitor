<script lang="ts">
	// noinspection ES6UnusedImports
	import * as Form from '$components/ui/form';
	import { formSchema as changePasswordFormSchema, type FormSchema } from './schema';
	import { type Infer, superForm, type SuperValidated } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import LineMdLoadingLoop from '~icons/line-md/loading-loop';
	import PasswordInput from '$components/password_input/PasswordInput.svelte';
	import { toast } from 'svelte-sonner';
	import { getTranslate, T } from '@tolgee/svelte';

	const { t } = getTranslate();

	const {
		data
	}: {
		data: SuperValidated<Infer<FormSchema>>;
	} = $props();

	const form = superForm(data, {
		validators: zodClient(changePasswordFormSchema),
		onUpdated: ({ form: f }) => {
			if (!f.valid) {
				toast.error($t('account_settings.fix_errors'));
			}
		}
	});

	const { form: formData, enhance, message, delayed } = form;
</script>

<form action="?/change_password" method="POST" use:enhance>
	<Form.Field {form} name="old_password">
		<Form.Control>
			{#snippet children({ props })}
				<Form.Label>
					<T keyName="account_settings.old_password" />
				</Form.Label>
				<PasswordInput
					{...props}
					bind:value={$formData.old_password}
					autocomplete="current-password"
				/>
			{/snippet}
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>
	<Form.Field {form} name="new_password">
		<Form.Control>
			{#snippet children({ props })}
				<Form.Label>
					<T keyName="account_settings.new_password" />
				</Form.Label>
				<PasswordInput {...props} bind:value={$formData.new_password} autocomplete="new-password" />
			{/snippet}
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>
	<Form.Field {form} name="new_password_confirm">
		<Form.Control>
			{#snippet children({ props })}
				<Form.Label>
					<T keyName="account_settings.new_password_confirm" />
				</Form.Label>
				<PasswordInput
					{...props}
					bind:value={$formData.new_password_confirm}
					autocomplete="new-password"
				/>
			{/snippet}
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>
	{#if $message}
		<div class="text-destructive">{$message}</div>
	{/if}
	<Form.Button class="mt-8 w-full">
		{#if !$delayed}
			<T keyName="account_settings.change_password_submit" />
		{:else}
			<LineMdLoadingLoop class="size-6" />
		{/if}
	</Form.Button>
</form>
