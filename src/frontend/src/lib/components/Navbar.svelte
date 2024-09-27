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
	import { invalidateAll } from '$app/navigation';
	import { page } from '$app/stores';
	import type { LoginStatus } from '../../app';

	export let loginStatus: LoginStatus;
	export let loggedInEmail: string | undefined;

	$: loggedIn = loginStatus === 'logged_in';

	async function refresh() {
		await invalidateAll();
	}

	$: isPublicPage = $page.url.pathname.startsWith('/public');
</script>

<nav class="flex h-20 flex-row items-center justify-between">
	<a class="ml-3 justify-self-start text-3xl" href={loggedIn ? '/' : undefined}>
		<PhHeartbeat />
	</a>
	<a class="absolute left-1/2 -translate-x-1/2 text-3xl" href={loggedIn ? '/' : undefined}>
		Monitor
	</a>
	<span class="mr-3 flex items-center gap-1 justify-self-end">
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
					<DropdownMenu.Item on:click={refresh}>
						<LucideRefreshCw class="mr-2 h-4 w-4" />
						Refresh
					</DropdownMenu.Item>
					{#if loggedIn}
						<DropdownMenu.Item href="/account_management">
							<LucideSettings class="mr-2 h-4 w-4" />
							Manage account
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
	</span>
</nav>
