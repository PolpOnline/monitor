<script lang="ts">
	import { Calendar as CalendarPrimitive } from 'bits-ui';
	import { cn } from '$lib/utils.js';

	type $$Props = CalendarPrimitive.HeadingProps;

	let className: $$Props['class'] = undefined;
	export { className as class };
</script>

<CalendarPrimitive.Heading
	{...$$restProps}
	class={cn("text-sm font-medium", className)}
	let:headingValue
>
	<slot {headingValue}>
		{headingValue}
	</slot>
</CalendarPrimitive.Heading>
