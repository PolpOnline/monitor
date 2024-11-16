<script lang="ts">
	// noinspection ES6UnusedImports
	import * as Form from '$components/ui/form';
	import { formSchema as changeLanguageFormSchema, type FormSchema } from './schema';
	import { type Infer, superForm, type SuperValidated } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import LineMdLoadingLoop from '~icons/line-md/loading-loop';
	import LanguageSelector from '$components/language_selector/LanguageSelector.svelte';
	import { toast } from 'svelte-sonner';
	import { getTranslate, T } from '@tolgee/svelte';

	const { t } = getTranslate();

	export let data: SuperValidated<Infer<FormSchema>>;
	export let languages: { value: string; label: string }[] = [
		{ value: 'en', label: 'English' },
		{ value: 'it', label: 'Italian' }
	];

	const form = superForm(data, {
		dataType: 'json',
		validators: zodClient(changeLanguageFormSchema),
		resetForm: false,
		onUpdated: ({ form: f }) => {
			if (f.valid) {
				toast.success($t('account_settings.language_changed_successfully'));
			} else {
				toast.error($t('account_settings.fix_errors'));
			}
		}
	});

	const { form: formData, enhance, message, delayed } = form;
</script>

<form action="?/change_language" method="POST" use:enhance>
	<Form.Field {form} name="language">
		<Form.Control>
			<LanguageSelector bind:value={$formData.language} {languages} class="w-full" />
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>
	{#if $message}
		<div class="text-destructive">{$message}</div>
	{/if}
	<Form.Button class="mt-8 w-full">
		{#if !$delayed}
			<T keyName="account_settings.change_language_submit" />
		{:else}
			<LineMdLoadingLoop class="h-6 w-6" />
		{/if}
	</Form.Button>
</form>
