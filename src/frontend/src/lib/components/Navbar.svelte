<script lang="ts">
	import PhHeartbeat from '~icons/ph/heartbeat';
	import { Button } from '$lib/components/ui/button';
	import HeroiconsMoon from '~icons/heroicons/moon';
	import HeroiconsSun from '~icons/heroicons/sun';
	import type { LoginStatus } from '../../../../backend/bindings/index';
	import { page } from '$app/stores';

	import { toggleMode } from 'mode-watcher';

	export let loginStatus: LoginStatus;

	const loggedIn = loginStatus === 'logged_in';
</script>

<nav class="flex h-20 flex-row items-center justify-between">
	<a class="ml-3 justify-self-start text-3xl" href={loggedIn ? '/' : undefined}>
		<PhHeartbeat />
	</a>
	<a class="absolute left-1/2 mx-2 -translate-x-1/2 text-3xl" href={loggedIn ? '/' : undefined}>
		Monitor
	</a>
	<span class="mr-3 flex items-center gap-1 justify-self-end">
		<Button on:click={toggleMode} size="icon" variant="outline">
			<HeroiconsSun
				class="h-[1.2rem] w-[1.2rem] rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0"
			/>
			<HeroiconsMoon
				class="absolute h-[1.2rem] w-[1.2rem] rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100"
			/>
			<span class="sr-only">Toggle theme</span>
		</Button>

		{#if $page.url.pathname !== '/login'}
			{#if loginStatus === 'logged_in'}
				<Button href="/logout" variant="destructive">Logout</Button>
			{:else}
				<Button href="/login">Login</Button>
			{/if}
		{/if}
	</span>
</nav>
