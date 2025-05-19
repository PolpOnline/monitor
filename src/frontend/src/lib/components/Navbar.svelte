<script lang="ts">
	import PhHeartbeat from '~icons/ph/heartbeat';
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
	import { page } from '$app/state';
	import type { LoginStatus } from '../../app';
	import { toast } from 'svelte-sonner';
	import { getTranslate, T } from '@tolgee/svelte';
	import DropdownMenuLinkItem from '$components/DropdownMenuLinkItem.svelte';
	import { buttonVariants } from '$lib/components/ui/button/index.js';
	import { onMount } from 'svelte';

	const { t } = getTranslate();

	const {
		loginStatus,
		loggedInEmail
	}: { loginStatus: LoginStatus; loggedInEmail: string | undefined } = $props();

	const loggedIn = $derived(loginStatus === 'logged_in');

	const isPublicPage = $derived(page.url.pathname.startsWith('/public'));

	let rKeyPressed = false;

	onMount(() => {
		addEventListener('keydown', keyDownHandler);
		addEventListener('keyup', keyUpHandler);
		return () => {
			removeEventListener('keydown', keyDownHandler);
			removeEventListener('keyup', keyUpHandler);
		};
	});

	async function keyDownHandler(event: KeyboardEvent) {
		if (event.isComposing) {
			return;
		}

		if (event.altKey && event.key.toLowerCase() === 'r' && !rKeyPressed) {
			rKeyPressed = true;
			await refresh();
		}
	}

	async function keyUpHandler(event: KeyboardEvent) {
		if (event.isComposing) {
			return;
		}

		if (event.altKey && event.key.toLowerCase() === 'r') {
			rKeyPressed = false;
		}
	}

	async function refresh() {
		// @ts-expect-error the component works fine
		toast($t('refreshing'), { icon: LineMdLoadingLoop });
		await invalidateAll();
		toast.success($t('refreshed'));
	}
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
			<DropdownMenu.Trigger class={buttonVariants({ size: 'icon' })} aria-label="Navbar Menu">
				<LucideUser />
			</DropdownMenu.Trigger>
			<DropdownMenu.Content>
				{#if loggedInEmail}
					<DropdownMenu.Label>{loggedInEmail}</DropdownMenu.Label>
					<DropdownMenu.Separator />
				{/if}
				<DropdownMenu.Group>
					{#if isPublicPage && !loggedIn}
						<DropdownMenuLinkItem href="/login">
							<LucideLogIn class="mr-2 size-4" />
							<T keyName="login" />
						</DropdownMenuLinkItem>
					{/if}
					<DropdownMenu.Item onclick={toggleMode}>
						{#if mode.current === 'dark'}
							<LucideSun class="mr-2 size-4" />
						{:else}
							<LucideMoon class="mr-2 size-4" />
						{/if}
						<span>
							<T keyName="toggle_theme" />
						</span>
					</DropdownMenu.Item>
					<DropdownMenu.Item onclick={refresh}>
						<LucideRefreshCw class="mr-2 size-4" />
						<T keyName="refresh" />
						<DropdownMenu.Shortcut>Alt+R</DropdownMenu.Shortcut>
					</DropdownMenu.Item>
					{#if loggedIn}
						<DropdownMenuLinkItem href="/account_settings">
							<LucideSettings class="mr-2 size-4" />
							<T keyName="account_settings" />
						</DropdownMenuLinkItem>
						<div data-sveltekit-preload-data="off">
							<DropdownMenuLinkItem class="text-red-600" href="/logout">
								<LucideLogOut class="mr-2 size-4" />
								<T keyName="logout" />
							</DropdownMenuLinkItem>
						</div>
					{/if}
				</DropdownMenu.Group>
				<DropdownMenu.Separator />
				<DropdownMenuLinkItem href="https://github.com/PolpOnline/monitor" target="_blank">
					<LucideGithub class="mr-2 size-4" />
					<T keyName="view_on_github" />
				</DropdownMenuLinkItem>
			</DropdownMenu.Content>
		</DropdownMenu.Root>
	</div>
</nav>
