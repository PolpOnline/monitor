<script lang="ts">
	import { Button } from '$components/ui/button';
	import { toast } from 'svelte-sonner';
	import { fly, type FlyParams } from 'svelte/transition';
	import { cubicIn } from 'svelte/easing';
	import LucideClipboardCopy from '~icons/lucide/clipboard-copy';
	import LineMdConfirm from '~icons/line-md/confirm';
	import Prism from 'prismjs';
	import 'prismjs/themes/prism-tomorrow.css';
	import { getTranslate } from '@tolgee/svelte';
	import { registerMikrotikLanguage } from './code_grammar/mikrotik';

	registerMikrotikLanguage();

	let {
		value = '',
		language = '',
		class: className = ''
	}: {
		value: string;
		language?: string;
		class?: string;
	} = $props();

	const { t } = getTranslate();

	const highlightedValue = $derived.by(() => {
		const grammar =
			Prism.languages[language] ?? Prism.languages.plaintext ?? Prism.languages.markup;
		return Prism.highlight(value, grammar, language);
	});

	let displayCheckMark = $state(false);

	async function copyValue() {
		try {
			await navigator.clipboard.writeText(value);
			toast.success($t('copied_to_clipboard'));
			displayCheckMark = true;
			setTimeout(() => (displayCheckMark = false), 3000);
		} catch (error) {
			console.error('Failed to copy code snippet', error);
			toast.error($t('error.copy_failed'));
		}
	}

	const flyInOptions: FlyParams = {
		delay: 0,
		duration: 300,
		easing: cubicIn,
		x: '-25%'
	};
</script>

<div class={`relative my-3 ${className}`}>
	<!-- svelte-ignore a11y_click_events_have_key_events -->
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div
		class="bg-muted/50 h-full w-full overflow-auto rounded-md border p-3 font-mono text-sm leading-relaxed"
		onclick={copyValue}
	>
		<!-- using @html to render the highlighted value is perfectly fine here, it cannot lead to XSS -->
		<!-- eslint-disable-next-line svelte/no-at-html-tags -->
		<code>{@html highlightedValue}</code>
	</div>
	<Button class="absolute right-2 bottom-2" onclick={copyValue} variant="secondary" size="icon">
		{#if !displayCheckMark}
			<div in:fly={flyInOptions}>
				<LucideClipboardCopy class="h-full w-full" />
			</div>
		{:else}
			<div in:fly={flyInOptions}>
				<LineMdConfirm class="h-full w-full" />
			</div>
		{/if}
	</Button>

	<!-- Hidden textarea ensures a selectable source when Prism markup is shown -->
	<textarea class="sr-only" aria-hidden="true" tabindex="-1" readonly>{value}</textarea>
</div>
