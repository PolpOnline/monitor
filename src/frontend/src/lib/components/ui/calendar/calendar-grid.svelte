<script lang="ts">
	import { Calendar as CalendarPrimitive } from 'bits-ui';
	import { cn } from '$lib/utils.js';

	type $$Props = CalendarPrimitive.GridProps;

	let className: $$Props['class'] = undefined;
	export { className as class };
</script>

<CalendarPrimitive.Grid {...$$restProps} class={cn('w-full border-collapse space-y-1', className)}>
	<slot />
</CalendarPrimitive.Grid>
