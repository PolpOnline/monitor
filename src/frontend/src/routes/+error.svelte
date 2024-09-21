<script lang="ts">
	import { page } from '$app/stores';
</script>

<div class="absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 text-center">
	<h1 class="text-bold text-9xl">{$page.status}</h1>
	<h2 class="text-light mt-4 text-5xl text-gray-500">{$page.error?.message}</h2>
</div>
