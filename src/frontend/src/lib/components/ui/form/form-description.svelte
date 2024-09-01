<script lang="ts">
	import type { HTMLAttributes } from 'svelte/elements';
	import { cn } from '$lib/utils.js';

	type $$Props = HTMLAttributes<HTMLSpanElement>;
	let className: string | undefined | null = undefined;
	export { className as class };
</script>

<FormPrimitive.Description
	{...$$restProps}
	class={cn('text-sm text-muted-foreground', className)}
	let:descriptionAttrs
>
	<slot {descriptionAttrs} />
</FormPrimitive.Description>
