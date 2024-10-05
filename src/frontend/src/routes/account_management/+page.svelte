<script lang="ts">
	import type { PageData } from './$types.js';
	import ChangePasswordForm from './change_password/ChangePasswordForm.svelte';
	import { title } from '$components/stores/title.store';
	import ChangeTimezoneForm from './change_timezone/ChangeTimezoneForm.svelte';
	// noinspection ES6UnusedImports
	import * as Card from '$components/ui/card';
	import LucideChevronLeft from '~icons/lucide/chevron-left';
	import { Button } from '$components/ui/button';
	title.set('Login');

	export let data: PageData;
</script>

<div class="mx-3">
	<div class="mb-5 mt-3 grid grid-cols-3">
		<Button href="/" variant="outline" size="icon">
			<LucideChevronLeft class="h-6 w-6" />
		</Button>
		<h1 class="text-center text-3xl font-bold">Account settings</h1>
		<div />
	</div>

	<div class="grid gap-6 md:grid-cols-2">
		<Card.Root>
			<Card.Header>
				<Card.Title>Change Password</Card.Title>
				<Card.Description>Update your account password here.</Card.Description>
			</Card.Header>
			<Card.Content>
				<ChangePasswordForm data={data.passwordForm} />
			</Card.Content>
		</Card.Root>

		<Card.Root>
			<Card.Header>
				<Card.Title>Change Timezone</Card.Title>
				<Card.Description>Select your preferred timezone.</Card.Description>
			</Card.Header>
			<Card.Content>
				<ChangeTimezoneForm data={data.timezoneForm} timezones={data.timezones} />
			</Card.Content>
		</Card.Root>
	</div>
</div>
