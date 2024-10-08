<script lang="ts">
	import PhHeartbeat from '~icons/ph/heartbeat';
	import { Button } from '$lib/components/ui/button';
	import LucideMoon from '~icons/lucide/moon';
	import LucideSun from '~icons/lucide/sun';
	import LucideLogIn from '~icons/lucide/log-in';
	import LucideLogOut from '~icons/lucide/log-out';
	// noinspection ES6UnusedImports
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import LucideUser from '~icons/lucide/user';
	import LucideSettings from '~icons/lucide/settings';
	import LucideRefreshCw from '~icons/lucide/refresh-cw';
	import { mode, toggleMode } from 'mode-watcher';
	import LucideGithub from '~icons/lucide/github';
	import LineMdLoadingLoop from '~icons/line-md/loading-loop';
	import { invalidateAll } from '$app/navigation';
	import { page } from '$app/stores';
	import type { LoginStatus } from '../../app';

	export let loginStatus: LoginStatus;
	export let loggedInEmail: string | undefined;

	$: loggedIn = loginStatus === 'logged_in';

	$: isPublicPage = $page.url.pathname.startsWith('/public');

	let isRefreshing = false;
</script>

<nav class="grid h-20 grid-cols-3">
	<div class="flex items-center">
		<a class="ml-3 text-3xl" href={loggedIn ? '/' : undefined}>
			<PhHeartbeat />
		</a>
	</div>
	<div class="flex items-center justify-center">
		<a href="/" class="text-3xl">Monitor</a>
	</div>
	<div class="mr-3 flex items-center gap-1 justify-self-end">
		<DropdownMenu.Root>
			<DropdownMenu.Trigger>
				<Button size="icon">
					<LucideUser />
				</Button>
			</DropdownMenu.Trigger>
			<DropdownMenu.Content>
				{#if loggedInEmail}
					<DropdownMenu.Label>{loggedInEmail}</DropdownMenu.Label>
					<DropdownMenu.Separator />
				{/if}
				<DropdownMenu.Group>
					{#if isPublicPage && !loggedIn}
						<DropdownMenu.Item href="/login">
							<LucideLogIn class="mr-2 h-4 w-4" />
							Login
						</DropdownMenu.Item>
					{/if}
					<DropdownMenu.Item on:click={toggleMode}>
						{#if $mode === 'dark'}
							<LucideSun class="mr-2 h-4 w-4" />
						{:else}
							<LucideMoon class="mr-2 h-4 w-4" />
						{/if}
						<span>Toggle theme</span>
					</DropdownMenu.Item>
					<DropdownMenu.Item
						on:click={async () => {
							isRefreshing = true;
							await invalidateAll();
							isRefreshing = false;
						}}
					>
						{#if !isRefreshing}
							<LucideRefreshCw class="mr-2 h-4 w-4" />
						{:else}
							<LineMdLoadingLoop class="mr-2 h-4 w-4" />
						{/if}
						Refresh
					</DropdownMenu.Item>
					{#if loggedIn}
						<DropdownMenu.Item href="/account_settings">
							<LucideSettings class="mr-2 h-4 w-4" />
							Account settings
						</DropdownMenu.Item>
						<DropdownMenu.Item
							class="text-red-600"
							href="/logout"
							data-sveltekit-preload-data="off"
						>
							<LucideLogOut class="mr-2 h-4 w-4" />
							Logout
						</DropdownMenu.Item>
					{/if}
				</DropdownMenu.Group>
				<DropdownMenu.Separator />
				<DropdownMenu.Item href="https://github.com/PolpOnline/monitor" target="_blank">
					<LucideGithub class="mr-2 h-4 w-4" />
					View on GitHub
				</DropdownMenu.Item>
			</DropdownMenu.Content>
		</DropdownMenu.Root>
	</div>
</nav>
