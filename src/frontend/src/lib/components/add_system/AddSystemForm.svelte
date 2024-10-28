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
	import { addSystemSheetOpen } from '$lib/components/stores/popovers.store';
	import LineMdLoadingLoop from '~icons/line-md/loading-loop';
	import LucidePencil from '~icons/lucide/pencil';
	import LucideClock from '~icons/lucide/clock';
	import LucidePlay from '~icons/lucide/play';
	import LucideMail from '~icons/lucide/mail';
	import LucideEye from '~icons/lucide/eye';
	// noinspection ES6UnusedImports
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';
	import { Button } from '$lib/components/ui/button/index.js';
	import DurationPicker from '$components/duration_picker/DurationPicker.svelte';
	import { toast } from 'svelte-sonner';
	import { getTranslate, T } from '@tolgee/svelte';

	const { t } = getTranslate();

	export let data: SuperValidated<Infer<FormSchema>>;

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

	export let delayed = form.delayed;

	let className = '';

	// noinspection ReservedWordAsName
	export { className as class };

	export let typeOfWrapper: 'sheet' | 'drawer' = 'sheet';
</script>

<form action="?/add_system" class={className} method="POST" use:enhance>
	<div class="space-y-9 p-4">
		<Form.Field {form} name="name">
			<Form.Control let:attrs>
				<Form.Label class="font-bold">
					<LucidePencil class="inline h-4 w-4" />
					<T keyName="add_system.name" />
				</Form.Label>
				<Input {...attrs} bind:value={$formData.name} />
			</Form.Control>
			<Form.FieldErrors />
		</Form.Field>

		<Form.Field {form} name="frequency">
			<Form.Control>
				<Form.Label>
					<LucideClock class="inline h-4 w-4" />
					<T keyName="add_system.check_frequency" />
				</Form.Label>
				<DurationPicker bind:value={$formData.frequency} defaultValue={{ hours: 0, minutes: 30 }} />
			</Form.Control>
			<Form.FieldErrors />
		</Form.Field>

		<Form.Field {form} name="starts_at">
			<Form.Control let:attrs>
				<Form.Label>
					<LucidePlay class="inline h-4 w-4" />
					<T keyName="add_system.starting_date_and_time" />
				</Form.Label>
				<DateTimePicker {...attrs} bind:value={$formData.starts_at} />
			</Form.Control>
			<Form.FieldErrors />
		</Form.Field>

		<Form.Field {form} name="down_after">
			<Form.Control let:attrs>
				<Form.Label>
					<LucideMail class="inline h-4 w-4" />
					<T keyName="add_system.send_email_after" />
				</Form.Label>
				<DurationPicker
					{...attrs}
					bind:value={$formData.down_after}
					defaultValue={{ hours: 2, minutes: 0 }}
				/>
			</Form.Control>
			<Form.FieldErrors />
		</Form.Field>

		<Form.Field {form} name="visibility">
			<Form.Control let:attrs>
				<Form.Label>
					<LucideEye class="inline h-4 w-4" />
					<T keyName="add_system.visibility" />
				</Form.Label>

				<div {...attrs}>
					<DropdownMenu.Root>
						<DropdownMenu.Trigger asChild let:builder>
							<Button variant="outline" builders={[builder]} class="w-full">
								{$formData.visibility === 'public'
									? $t('add_system.public')
									: $t('add_system.private')}
							</Button>
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
