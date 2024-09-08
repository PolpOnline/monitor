<script lang="ts">
	import type { PageData } from './$types.js';
	import ChangePasswordForm from './ChangePasswordForm.svelte';
	import { title } from '$components/stores/title.store';

	title.set('Login');

	export let data: PageData;
</script>

<div class="mx-auto mt-10">
	<h1 class="mb-5 text-center text-3xl font-bold">Account management</h1>

	<div class="mx-4">
		<div class="mx-auto max-w-96">
			<h2 class="mb-5 text-center text-xl font-bold">Change your password</h2>
			<ChangePasswordForm data={data.form} />
		</div>
	</div>
</div>
