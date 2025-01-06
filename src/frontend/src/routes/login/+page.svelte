<script lang="ts">
	import type { PageData } from './$types.js';
	import LoginForm from './LoginForm.svelte';
	import { title } from '$components/stores/title.store';
	import { getTranslate, T } from '@tolgee/svelte';

	const { t } = getTranslate();

	title.set($t('login.login_or_signup'));

	let { data }: { data: PageData } = $props();
</script>

<main class="h-screen-not-navbar flex flex-col items-center justify-center">
	<div class="max-w-96">
		<h1 class="mb-5 text-center text-3xl font-bold">
			<T keyName="login.login_or_signup" />
		</h1>
		<div class="w-11/12 min-w-96">
			<LoginForm data={data.form} />
		</div>
	</div>
</main>

<style lang="postcss">
	.h-screen-not-navbar {
		height: calc(100svh - 5rem);
	}
</style>
