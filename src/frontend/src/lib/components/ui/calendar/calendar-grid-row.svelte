<script lang="ts">
	import { Calendar as CalendarPrimitive } from 'bits-ui';
	import { cn } from '$lib/utils.js';

	type $$Props = CalendarPrimitive.GridRowProps;

	let className: $$Props['class'] = undefined;
	export { className as class };
</script>

<CalendarPrimitive.GridRow {...$$restProps} class={cn('flex', className)}>
	<slot />
</CalendarPrimitive.GridRow>
