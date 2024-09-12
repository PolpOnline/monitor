<script lang="ts">
	import { Drawer as DrawerPrimitive } from 'vaul-svelte';

	type $$Props = DrawerPrimitive.Props;
	export let shouldScaleBackground: $$Props['shouldScaleBackground'] = true;
	export let open: $$Props['open'] = false;
	export let activeSnapPoint: $$Props['activeSnapPoint'] = undefined;
</script>

<DrawerPrimitive.NestedRoot {...$$restProps} bind:activeSnapPoint bind:open {shouldScaleBackground}>
	<slot />
</DrawerPrimitive.NestedRoot>
