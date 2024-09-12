<script lang="ts">
	import type { HTMLAttributes } from 'svelte/elements';
	import { cn } from '$lib/utils.js';

	type $$Props = HTMLAttributes<HTMLDivElement> & {
		el?: HTMLDivElement;
	};

	export let el: $$Props['el'] = undefined;
	let className: $$Props['class'] = undefined;
	export { className as class };
</script>

<div {...$$restProps} bind:this={el} class={cn("mt-auto flex flex-col gap-2 p-4", className)}>
	<slot />
</div>
