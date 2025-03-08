<script lang="ts">
	import type { HTMLInputAttributes } from 'svelte/elements';
	import LucideEye from '~icons/lucide/eye';
	import LucideEyeOff from '~icons/lucide/eye-off';
	import type { WithElementRef } from 'bits-ui';
	import { Input } from '$components/ui/input';

	let {
		value = $bindable(),
		class: className,
		...restProps
	}: WithElementRef<HTMLInputAttributes> = $props();

	let showPassword = $state(false);
</script>

<div class="relative flex items-center">
	<Input class={className} bind:value {...restProps} type={showPassword ? 'text' : 'password'} />
	<button
		type="button"
		class="text-foreground hover:text-foreground/65 absolute right-2 cursor-pointer"
		onclick={() => (showPassword = !showPassword)}
		tabindex="-1"
	>
		{#if showPassword}
			<LucideEye />
		{:else}
			<LucideEyeOff />
		{/if}
	</button>
</div>
