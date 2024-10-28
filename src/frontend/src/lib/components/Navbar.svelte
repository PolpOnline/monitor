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
	import { toast } from 'svelte-sonner';
	import { T, getTranslate } from '@tolgee/svelte';

	const { t } = getTranslate();

	export let loginStatus: LoginStatus;
	export let loggedInEmail: string | undefined;

	$: loggedIn = loginStatus === 'logged_in';

	$: isPublicPage = $page.url.pathname.startsWith('/public');
</script>

<nav class="grid h-20 grid-cols-3">
	<div class="flex items-center">
		<a class="ml-3 text-3xl" href={loggedIn ? '/' : undefined} aria-label="Go to home">
			<PhHeartbeat />
		</a>
	</div>
	<div class="flex items-center justify-center">
		<a href="/" class="text-3xl">Monitor</a>
	</div>
	<div class="mr-3 flex items-center gap-1 justify-self-end">
		<DropdownMenu.Root>
			<DropdownMenu.Trigger>
				<Button size="icon" aria-label="Navbar Menu">
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
							<T keyName="login" />
						</DropdownMenu.Item>
					{/if}
					<DropdownMenu.Item on:click={toggleMode}>
						{#if $mode === 'dark'}
							<LucideSun class="mr-2 h-4 w-4" />
						{:else}
							<LucideMoon class="mr-2 h-4 w-4" />
						{/if}
						<span>
							<T keyName="toggle_theme" />
						</span>
					</DropdownMenu.Item>
					<DropdownMenu.Item
						on:click={async () => {
							toast($t('refreshing'), { icon: LineMdLoadingLoop });
							await invalidateAll();
							toast.success($t('refreshed'));
						}}
					>
						<LucideRefreshCw class="mr-2 h-4 w-4" />
						<T keyName="refresh" />
					</DropdownMenu.Item>
					{#if loggedIn}
						<DropdownMenu.Item href="/account_settings">
							<LucideSettings class="mr-2 h-4 w-4" />
							<T keyName="account_settings" />
						</DropdownMenu.Item>
						<DropdownMenu.Item
							class="text-red-600"
							href="/logout"
							data-sveltekit-preload-data="off"
						>
							<LucideLogOut class="mr-2 h-4 w-4" />
							<T keyName="logout" />
						</DropdownMenu.Item>
					{/if}
				</DropdownMenu.Group>
				<DropdownMenu.Separator />
				<DropdownMenu.Item href="https://github.com/PolpOnline/monitor" target="_blank">
					<LucideGithub class="mr-2 h-4 w-4" />
					<T keyName="view_on_github" />
				</DropdownMenu.Item>
			</DropdownMenu.Content>
		</DropdownMenu.Root>
	</div>
</nav>
