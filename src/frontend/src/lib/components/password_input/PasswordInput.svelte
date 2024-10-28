<script lang="ts">
	import type { HTMLInputAttributes } from 'svelte/elements';
	import type { InputEvents } from './index.js';
	import { cn } from '$lib/utils.js';
	import LucideEye from '~icons/lucide/eye';
	import LucideEyeOff from '~icons/lucide/eye-off';

	type $$Props = HTMLInputAttributes;
	// eslint-disable-next-line @typescript-eslint/no-unused-vars
	type $$Events = InputEvents;

	let className: $$Props['class'] = undefined;
	export let value: $$Props['value'] = undefined;
	// noinspection ReservedWordAsName
	export { className as class };

	// Workaround for https://github.com/sveltejs/svelte/issues/9305
	// Fixed in Svelte 5, but not backported to 4.x.
	export let readonly: $$Props['readonly'] = undefined;

	let showPassword = false;

	function togglePasswordVisibility() {
		showPassword = !showPassword;
	}
</script>

<div class="relative flex items-center">
	<input
		class={cn(
			'flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50',
			className
		)}
		on:input={(e) => (value = e.currentTarget.value)}
		{value}
		{readonly}
		on:blur
		on:change
		on:click
		on:focus
		on:focusin
		on:focusout
		on:keydown
		on:keypress
		on:keyup
		on:mouseover
		on:mouseenter
		on:mouseleave
		on:mousemove
		on:paste
		on:input
		on:wheel|passive
		{...$$restProps}
		type={showPassword ? 'text' : 'password'}
	/>
	<button
		type="button"
		class="absolute right-2 cursor-pointer text-foreground hover:text-foreground/65"
		on:click={togglePasswordVisibility}
		tabindex="-1"
	>
		{#if showPassword}
			<LucideEye />
		{:else}
			<LucideEyeOff />
		{/if}
	</button>
</div>
