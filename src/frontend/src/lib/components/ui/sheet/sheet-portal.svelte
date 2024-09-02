<script lang="ts">
	import { Dialog as SheetPrimitive } from 'bits-ui';
	import { cn } from '$lib/utils.js';

	type $$Props = SheetPrimitive.PortalProps;

	let className: $$Props['class'] = undefined;
	export { className as class };
</script>

<SheetPrimitive.Portal {...$$restProps} class={cn(className)}>
	<slot />
</SheetPrimitive.Portal>
