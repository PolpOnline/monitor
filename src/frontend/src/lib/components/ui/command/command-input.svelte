<script lang="ts">
	import { Command as CommandPrimitive } from "bits-ui";
	import { cn } from "$lib/utils.js";
	import SearchIcon from '@lucide/svelte/icons/search';

	let {
		ref = $bindable(null),
		class: className,
		value = $bindable(""),
		...restProps
	}: CommandPrimitive.InputProps = $props();
</script>

<div data-slot="command-input-wrapper" class="border-border border-b px-3 py-2">
	<div class="relative">
		<SearchIcon
			class="text-muted-foreground pointer-events-none absolute top-1/2 left-0 size-4 -translate-y-1/2"
		/>
		<CommandPrimitive.Input
			{value}
			data-slot="command-input"
			class={cn(
				"w-full bg-transparent py-1 pr-2 pl-6 text-sm outline-none placeholder:text-muted-foreground disabled:cursor-not-allowed disabled:opacity-50",
				className
			)}
			{...restProps}
		>
			{#snippet child({ props })}
				<input {...props} bind:value bind:this={ref} />
			{/snippet}
		</CommandPrimitive.Input>
	</div>
</div>
