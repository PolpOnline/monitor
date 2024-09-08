<script lang="ts">
	import PhHeartbeat from '~icons/ph/heartbeat';
	import { Button } from '$lib/components/ui/button';
	import LucideMoon from '~icons/lucide/moon';
	import LucideSun from '~icons/lucide/sun';
	import LucideLogOut from '~icons/lucide/log-out';
	import type { LoginStatus } from '../../../../backend/bindings/index';
	// import { page } from '$app/stores';
	// noinspection ES6UnusedImports
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import LucideUser from '~icons/lucide/user';
	import LucideSettings from '~icons/lucide/settings';
	import LucideRefreshCw from '~icons/lucide/refresh-cw';
	import { mode } from 'mode-watcher';
	import { invalidate, invalidateAll } from '$app/navigation';

	import { toggleMode } from 'mode-watcher';
	import { API_URL } from '$lib/api/api';

	export let loginStatus: LoginStatus;

	const loggedIn = loginStatus === ('logged_in' as LoginStatus);

	async function refresh() {
		// await invalidate(`${API_URL}/list_systems`);
		await invalidateAll();
	}
</script>

<nav class="flex h-20 flex-row items-center justify-between">
	<a class="ml-3 justify-self-start text-3xl" href={loggedIn ? '/' : undefined}>
		<PhHeartbeat />
	</a>
	<a class="absolute left-1/2 mx-2 -translate-x-1/2 text-3xl" href={loggedIn ? '/' : undefined}>
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
				<DropdownMenu.Group>
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
					<DropdownMenu.Item href="/account_management">
						<LucideSettings class="mr-2 h-4 w-4" />
						Manage account
					</DropdownMenu.Item>
					<DropdownMenu.Item class="text-red-600" href="/logout" data-sveltekit-preload-data="off">
						<LucideLogOut class="mr-2 h-4 w-4" />
						Logout
					</DropdownMenu.Item>
				</DropdownMenu.Group>
			</DropdownMenu.Content>
		</DropdownMenu.Root>
	</span>
</nav>
