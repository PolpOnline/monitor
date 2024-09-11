<script lang="ts">
	// noinspection ES6UnusedImports
	import { Textarea } from '$lib/components/ui/textarea';
	import { Button } from '$components/ui/button';
	import LucideClipboardCopy from '~icons/lucide/clipboard-copy';
	import LucideCheck from '~icons/lucide/check';

	let className = '';
	// noinspection ReservedWordAsName
	export { className as class };

	export let value = '';

	let displayCheckMark = false;

	function copyValue() {
		navigator.clipboard.writeText(value);
		displayCheckMark = true;
		setTimeout(() => {
			displayCheckMark = false;
		}, 1000);
	}
</script>

<div class="relative my-3">
	<Textarea bind:value readonly class={className} />
	<Button on:click={copyValue} class="absolute bottom-2 right-2">
		{#if !displayCheckMark}
			<LucideClipboardCopy class="h-6 w-6" />
		{:else}
			<LucideCheck class="h-6 w-6" />
		{/if}
	</Button>
</div>
