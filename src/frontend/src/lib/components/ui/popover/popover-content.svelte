<script lang="ts">
	import { Popover as PopoverPrimitive } from 'bits-ui';
	import { cn, flyAndScale } from '$lib/utils.js';

	type $$Props = PopoverPrimitive.ContentProps;
	let className: $$Props['class'] = undefined;
	export let transition: $$Props['transition'] = flyAndScale;
	export let transitionConfig: $$Props['transitionConfig'] = undefined;
	export { className as class };
</script>

<PopoverPrimitive.Content
	{...$$restProps}
	class={cn(
		"bg-popover text-popover-foreground z-50 w-72 rounded-md border p-4 shadow-md outline-none",
		className
	)}
	{transition}
	{transitionConfig}
>
	<slot />
</PopoverPrimitive.Content>
