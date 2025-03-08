<script lang="ts">
	// noinspection ES6UnusedImports
	import * as Sheet from '$lib/components/ui/sheet/index.js';
	import { Input } from '$lib/components/ui/input/index.js';
	import DateTimePicker from '$components/date_time_picker/DateTimePicker.svelte';
	// noinspection ES6UnusedImports
	import * as Form from '$lib/components/ui/form';
	// noinspection ES6UnusedImports
	import * as Drawer from '$lib/components/ui/drawer';
	import { formSchema, type FormSchema } from './schema';
	import { type Infer, superForm, type SuperValidated } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import { addSystemSheetOpen } from '$lib/stores/popovers.store';
	import LineMdLoadingLoop from '~icons/line-md/loading-loop';
	import LucidePencil from '~icons/lucide/pencil';
	import LucideClock from '~icons/lucide/clock';
	import LucidePlay from '~icons/lucide/play';
	import LucideMail from '~icons/lucide/mail';
	import LucideEye from '~icons/lucide/eye';
	// noinspection ES6UnusedImports
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';
	import { buttonVariants } from '$lib/components/ui/button/index.js';
	import DurationPicker from '$components/duration_picker/DurationPicker.svelte';
	import { toast } from 'svelte-sonner';
	import { getTranslate, T } from '@tolgee/svelte';
	import { cn } from '$lib/utils';

	const { t } = getTranslate();

	let {
		data,
		class: className = '',
		typeOfWrapper = 'sheet'
	}: {
		data: SuperValidated<Infer<FormSchema>>;
		class?: string;
		typeOfWrapper: 'sheet' | 'drawer';
	} = $props();

	// noinspection JSUnusedGlobalSymbols
	const form = superForm(data, {
		validators: zodClient(formSchema),
		dataType: 'json',
		onUpdated: ({ form: f }) => {
			if (f.valid) {
				$addSystemSheetOpen = false;
				toast.success($t('add_system.system_added_successfully'));
			}
		},
		invalidateAll: true
	});

	const { form: formData, enhance } = form;

	const delayed = form.delayed;

	let dateTimePicker: { value: string } | undefined = $state(); // bound to the DateTimePicker component

	let starts_at = $derived(dateTimePicker ? dateTimePicker.value : '');

	$effect(() => {
		$formData.starts_at = starts_at;
	});

	let durationPickerFrequency: { value: number } | undefined = $state(); // bound to the DurationPicker component

	let frequency = $derived(durationPickerFrequency ? durationPickerFrequency.value : 0);

	$effect(() => {
		$formData.frequency = frequency;
	});

	let durationPickerDownAfter: { value: number } | undefined = $state(); // bound to the DurationPicker component

	let down_after = $derived(durationPickerDownAfter ? durationPickerDownAfter.value : 0);

	$effect(() => {
		$formData.down_after = down_after;
	});
</script>

<form action="?/add_system" class={className} method="POST" use:enhance>
	<div class="space-y-9 p-4">
		<Form.Field {form} name="name">
			<Form.Control>
				{#snippet children({ props })}
					<Form.Label class="font-bold">
						<LucidePencil class="inline h-4 w-4" />
						<T keyName="add_system.name" />
					</Form.Label>
					<Input {...props} bind:value={$formData.name} />
				{/snippet}
			</Form.Control>
			<Form.FieldErrors />
		</Form.Field>

		<Form.Field {form} name="frequency">
			<Form.Control>
				{#snippet children()}
					<Form.Label>
						<LucideClock class="inline h-4 w-4" />
						<T keyName="add_system.check_frequency" />
					</Form.Label>
					<DurationPicker
						bind:this={durationPickerFrequency}
						defaultValue={{ hours: 4, minutes: 0 }}
					/>
				{/snippet}
			</Form.Control>
			<Form.FieldErrors />
		</Form.Field>

		<Form.Field {form} name="starts_at">
			<Form.Control>
				{#snippet children()}
					<Form.Label>
						<LucidePlay class="inline h-4 w-4" />
						<T keyName="add_system.starting_date_and_time" />
					</Form.Label>
					<DateTimePicker bind:this={dateTimePicker} />
				{/snippet}
			</Form.Control>
			<Form.FieldErrors />
		</Form.Field>

		<Form.Field {form} name="down_after">
			<Form.Control>
				{#snippet children()}
					<Form.Label>
						<LucideMail class="inline h-4 w-4" />
						<T keyName="add_system.send_email_after" />
					</Form.Label>
					<DurationPicker
						bind:this={durationPickerDownAfter}
						defaultValue={{ hours: 4, minutes: 0 }}
					/>
				{/snippet}
			</Form.Control>
			<Form.FieldErrors />
		</Form.Field>

		<Form.Field {form} name="visibility">
			<Form.Control>
				{#snippet children({ props })}
					<Form.Label>
						<LucideEye class="inline h-4 w-4" />
						<T keyName="add_system.visibility" />
					</Form.Label>

					<div {...props}>
						<DropdownMenu.Root>
							<DropdownMenu.Trigger class={cn(buttonVariants({ variant: 'outline' }), 'w-full')}>
								{$formData.visibility === 'public'
									? $t('add_system.public')
									: $t('add_system.private')}
							</DropdownMenu.Trigger>
							<DropdownMenu.Content class="w-[95%]">
								<DropdownMenu.RadioGroup bind:value={$formData.visibility}>
									<DropdownMenu.RadioItem value="public">
										<T keyName="add_system.public" />
									</DropdownMenu.RadioItem>
									<DropdownMenu.RadioItem value="private">
										<T keyName="add_system.private" />
									</DropdownMenu.RadioItem>
								</DropdownMenu.RadioGroup>
							</DropdownMenu.Content>
						</DropdownMenu.Root>
					</div>
				{/snippet}
			</Form.Control>
			<Form.FieldErrors />
		</Form.Field>
	</div>

	{#if typeOfWrapper === 'sheet'}
		<Sheet.Footer>
			<Form.Button>
				{#if !$delayed}
					<T keyName="add_system.add_system_submit" />
				{:else}
					<LineMdLoadingLoop class="h-6 w-6" />
				{/if}
			</Form.Button>
		</Sheet.Footer>
	{:else}
		<Drawer.Footer>
			<Form.Button>
				{#if !$delayed}
					<T keyName="add_system.add_system_submit" />
				{:else}
					<LineMdLoadingLoop class="h-6 w-6" />
				{/if}
			</Form.Button>
		</Drawer.Footer>
	{/if}
</form>
