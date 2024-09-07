<script lang="ts">
	import type { PageData } from './$types.js';
	import LoginForm from './login-form.svelte';
	import { title } from '$components/stores/title.store';

	title.set('Login');

	export let data: PageData;
</script>

<div class="absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2">
	<h1 class="mb-5 text-center text-3xl font-bold">Login or Sign up</h1>
	<div class="w-11/12 min-w-96">
		<LoginForm data={data.form} />
	</div>
</div>
