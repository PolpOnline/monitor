<script lang="ts">
	import { Button } from '$lib/components/ui/button/index.js';
	import { page } from '$app/stores';
	import LucideChevronLeft from '~icons/lucide/chevron-left';
	import LucideChevronRight from '~icons/lucide/chevron-right';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import { cn } from '$lib/utils';
	import { type ClassValue } from 'clsx';

	const currentPage = $derived(() => Number($page.url.searchParams.get('page')) || 0);

	const prevPageHref = $derived(() => (currentPage() > 1 ? `?page=${currentPage() - 1}` : `?`));
	const nextPageHref = $derived(() => `?page=${currentPage() + 1}`);

	onMount(() => {
		document.addEventListener('keydown', keyHandler);
		return () => {
			document.removeEventListener('keydown', keyHandler);
		};
	});

	function keyHandler(event: KeyboardEvent) {
		if (event.key === 'ArrowRight' && currentPage() > 0) {
			goto(prevPageHref());
		}
		if (event.key === 'ArrowLeft') {
			goto(nextPageHref());
		}
	}

	const { class: className }: { class: ClassValue } = $props();
</script>

<div class={cn(className, 'flex items-center justify-between')}>
	<Button
		variant="outline"
		size="icon"
		href={nextPageHref()}
		data-sveltekit-preload-data="hover"
		data-sveltekit-preload-code="eager"
		data-sveltekit-replacestate
		aria-label="Go back in time"
	>
		<LucideChevronLeft />
	</Button>
	<!-- We cannot set `disabled` on a button to which we passed a href, so we need to do this -->
	{#if currentPage() !== 0}
		<Button
			variant="outline"
			size="icon"
			href={prevPageHref()}
			data-sveltekit-preload-data="hover"
			data-sveltekit-preload-code="eager"
			data-sveltekit-replacestate
			aria-label="Go forward in time"
		>
			<LucideChevronRight />
		</Button>
	{:else}
		<Button variant="outline" size="icon" disabled aria-label="Go forward in time">
			<LucideChevronRight />
		</Button>
	{/if}
</div>
