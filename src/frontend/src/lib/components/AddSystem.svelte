<script lang="ts">
	// noinspection ES6UnusedImports
	import * as Sheet from '$lib/components/ui/sheet/index.js';
	import { Button } from '$lib/components/ui/button/index.js';
	import { Input } from '$lib/components/ui/input/index.js';
	import { Label } from '$lib/components/ui/label/index.js';
	import HeroiconsPlus20Solid from '~icons/heroicons/plus-20-solid';
</script>

<Sheet.Root>
	<Sheet.Trigger asChild let:builder>
		<Button builders={[builder]} class="fixed bottom-7 right-7 h-12 w-12">
			<HeroiconsPlus20Solid class="h-6 w-6" />
		</Button>
	</Sheet.Trigger>
	<Sheet.Content side="right">
		<Sheet.Header>
			<Sheet.Title>Add a system</Sheet.Title>
			<Sheet.Description>
				Add a new system to the monitor by filling out the form below.
			</Sheet.Description>
		</Sheet.Header>
		<div class="grid gap-4 py-4">
			<div class="grid grid-cols-4 items-center gap-4">
				<Label class="text-right" for="test1">Test</Label>
				<Input class="col-span-3" id="test1" value="Test text" />
			</div>
			<div class="grid grid-cols-4 items-center gap-4">
				<Label class="text-right" for="test2">Test 2</Label>
				<Input class="col-span-3" id="test2" value="Test 2 text" />
			</div>
		</div>
		<Sheet.Footer>
			<Sheet.Close asChild let:builder>
				<Button builders={[builder]} type="submit">Save changes</Button>
			</Sheet.Close>
		</Sheet.Footer>
	</Sheet.Content>
</Sheet.Root>
