<script lang="ts">
	// noinspection ES6UnusedImports
	import * as Form from '$components/ui/form';
	import { formSchema as changeTimezoneFormSchema, type FormSchema } from './schema';
	import { type Infer, superForm, type SuperValidated } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import LineMdLoadingLoop from '~icons/line-md/loading-loop';
	import TimezoneSelector from '$components/timezone_selector/TimezoneSelector.svelte';
	import { toast } from 'svelte-sonner';
	import { getTranslate, T } from '@tolgee/svelte';
	import { timezones } from '$lib/static_data/timezones';

	const { t } = getTranslate();

	const {
		data
	}: {
		data: SuperValidated<Infer<FormSchema>>;
	} = $props();

	const form = superForm(data, {
		dataType: 'json',
		validators: zodClient(changeTimezoneFormSchema),
		resetForm: false,
		onUpdated: ({ form: f }) => {
			if (f.valid) {
				toast.success($t('account_settings.timezone_changed_successfully'));
			} else {
				toast.error($t('account_settings.fix_errors'));
			}
		}
	});

	const { form: formData, enhance, message, delayed } = form;
</script>

<form action="?/change_timezone" method="POST" use:enhance>
	<Form.Field {form} name="timezone">
		<Form.Control>
			<TimezoneSelector bind:value={$formData.timezone} {timezones} class="w-full" />
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>
	{#if $message}
		<div class="text-destructive">{$message}</div>
	{/if}
	<Form.Button class="mt-8 w-full">
		{#if !$delayed}
			<T keyName="account_settings.change_timezone_submit" />
		{:else}
			<LineMdLoadingLoop class="size-6" />
		{/if}
	</Form.Button>
</form>
