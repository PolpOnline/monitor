<script lang="ts">
	// noinspection ES6UnusedImports
	import * as Form from '$lib/components/ui/form';
	import { Input } from '$lib/components/ui/input';
	import { formSchema, type FormSchema } from './schema';
	import { type Infer, superForm, type SuperValidated } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import LineMdLoadingLoop from '~icons/line-md/loading-loop';
	import PasswordInput from '$components/password_input/PasswordInput.svelte';
	import { toast } from 'svelte-sonner';
	import { getTranslate, T } from '@tolgee/svelte';
	import { goto } from '$app/navigation';

	const { t } = getTranslate();

	const {
		data
	}: {
		data: SuperValidated<Infer<FormSchema>>;
	} = $props();

	const form = superForm(data, {
		validators: zodClient(formSchema),
		onUpdated: ({ form: f }) => {
			if (f.valid) {
				toast.success($t('login.login_successful'));

				goto('/');
			}
		}
	});

	const { form: formData, enhance, message, delayed } = form;
</script>

<form method="POST" use:enhance>
	<Form.Field {form} name="email">
		<Form.Control>
			{#snippet children({ props })}
				<Form.Label>
					<T keyName="login.email" />
				</Form.Label>
				<Input {...props} bind:value={$formData.email} type="email" autocomplete="username" />
			{/snippet}
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>
	<Form.Field {form} name="password">
		<Form.Control>
			{#snippet children({ props })}
				<Form.Label>
					<T keyName="login.password" />
				</Form.Label>
				<PasswordInput {...props} bind:value={$formData.password} autocomplete="current-password" />
			{/snippet}
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>
	{#if $message}
		<div class="text-destructive">{$message}</div>
	{/if}
	<Form.Button class="mt-8 w-full">
		{#if !$delayed}
			<T keyName="login.login" />
		{:else}
			<LineMdLoadingLoop class="size-6" />
		{/if}
	</Form.Button>
</form>
