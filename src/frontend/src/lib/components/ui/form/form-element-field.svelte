<script context="module" lang="ts">
	import type { FormPathLeaves, SuperForm } from 'sveltekit-superforms';

	type T = Record<string, unknown>;
	type U = FormPathLeaves<T>;
</script>

<script generics="T extends Record<string, unknown>, U extends FormPathLeaves<T>" lang="ts">
	import type { HTMLAttributes } from 'svelte/elements';
	import * as FormPrimitive from 'formsnap';
	import { cn } from '$lib/utils.js';

	type $$Props = FormPrimitive.ElementFieldProps<T, U> & HTMLAttributes<HTMLElement>;

	export let form: SuperForm<T>;
	export let name: U;

	let className: $$Props['class'] = undefined;
	export { className as class };
</script>

<FormPrimitive.ElementField {form} let:constraints let:errors let:tainted let:value {name}>
	<div class={cn("space-y-2", className)}>
		<slot {constraints} {errors} {tainted} {value} />
	</div>
</FormPrimitive.ElementField>
