<script lang="ts">
	import type { PageData } from './$types.js';
	import LoginForm from './login-form.svelte';

	export let data: PageData;
</script>

<div class="absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2">
	<div class="max-w-96">
		<LoginForm data={data.form} />
	</div>
</div>
