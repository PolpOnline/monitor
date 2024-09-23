<script lang="ts">
	import type { PageData } from './$types.js';
	import LoginForm from './LoginForm.svelte';
	import { title } from '$components/stores/title.store';

	title.set('Login');

	export let data: PageData;
</script>

<div class="h-screen-not-navbar flex flex-col items-center justify-center">
	<div class="max-w-96">
		<h1 class="mb-5 text-center text-3xl font-bold">Login or Sign up</h1>
		<div class="w-11/12 min-w-96">
			<LoginForm data={data.form} />
		</div>
	</div>
</div>

<style lang="postcss">
	.h-screen-not-navbar {
		height: calc(100vh - 5rem);
	}
</style>
