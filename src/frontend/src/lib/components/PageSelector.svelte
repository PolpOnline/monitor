<script lang="ts">
	import { Button } from '$lib/components/ui/button/index.js';
	import { page } from '$app/stores';
	import LucideChevronLeft from '~icons/lucide/chevron-left';
	import LucideChevronRight from '~icons/lucide/chevron-right';

	$: currentPage = Number($page.url.searchParams.get('page')) || 0;

	$: prevPageHref = currentPage > 1 ? `?page=${currentPage - 1}` : `?`;
</script>

<div class="flex items-center justify-between">
	<Button variant="outline" size="icon" href="?page={currentPage + 1}">
		<LucideChevronLeft />
	</Button>
	<!-- We cannot set `disabled` on a button to which we passed a href, so we need to do this -->
	{#if currentPage !== 0}
		<Button variant="outline" size="icon" href={prevPageHref}>
			<LucideChevronRight />
		</Button>
	{:else}
		<Button variant="outline" size="icon" disabled>
			<LucideChevronRight />
		</Button>
	{/if}
</div>
