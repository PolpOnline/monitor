<script lang="ts">
	// noinspection ES6UnusedImports
	import { Textarea } from '$lib/components/ui/textarea';
	import { Button } from '$components/ui/button';
	import LucideClipboardCopy from '~icons/lucide/clipboard-copy';
	import LineMdConfirm from '~icons/line-md/confirm';
	import { toast } from 'svelte-sonner';
	import { fly, type FlyParams } from 'svelte/transition';
	import { cubicIn } from 'svelte/easing';
	import { getTranslate } from '@tolgee/svelte';

	const { t } = getTranslate();

	let className = '';
	// noinspection ReservedWordAsName
	export { className as class };

	export let value = '';

	let displayCheckMark = false;

	function copyValue() {
		navigator.clipboard.writeText(value);

		toast.success($t('copied_to_clipboard'));

		displayCheckMark = true;
		setTimeout(() => {
			displayCheckMark = false;
		}, 3000);
	}

	const flyInOptions: FlyParams = {
		delay: 0,
		duration: 300,
		easing: cubicIn,
		x: '-25%'
	};
</script>

<div class="relative my-3">
	<Textarea bind:value class={className} readonly />
	<Button class="absolute bottom-2 right-2" on:click={copyValue} variant="secondary">
		{#if !displayCheckMark}
			<div in:fly={flyInOptions} class="h-6 w-6">
				<LucideClipboardCopy class="h-full w-full" />
			</div>
		{:else}
			<div in:fly={flyInOptions} class="h-6 w-6">
				<LineMdConfirm class="h-full w-full" />
			</div>
		{/if}
	</Button>
</div>
