<script lang="ts">
	import type { PageData } from './$types.js';
	import ChangePasswordForm from './change_password/ChangePasswordForm.svelte';
	import { title } from '$components/stores/title.store';
	import ChangeTimezoneForm from './change_timezone/ChangeTimezoneForm.svelte';

	title.set('Login');

	export let data: PageData;
</script>

<div class="mx-auto mt-10">
	<h1 class="mb-5 text-center text-3xl font-bold">Account management</h1>

	<div class="mx-4">
		<div class="mx-auto max-w-96">
			<h2 class="mb-5 text-center text-xl font-bold">Change your password</h2>
			<ChangePasswordForm data={data.passwordForm} />

			<h2 class="mb-5 mt-10 text-center text-xl font-bold">Change your timezone</h2>
			<ChangeTimezoneForm data={data.timezoneForm} />
		</div>
	</div>
</div>
