<script lang="ts">
	import type { PageData } from './$types.js';
	import ChangePasswordForm from './change_password/ChangePasswordForm.svelte';
	import { title } from '$lib/stores/title.store';
	import ChangeTimezoneForm from './change_timezone/ChangeTimezoneForm.svelte';
	import ChangeLanguageForm from './change_language/ChangeLanguageForm.svelte';
	// noinspection ES6UnusedImports
	import * as Card from '$components/ui/card';
	import LucideChevronLeft from '~icons/lucide/chevron-left';
	import LucideRectangleEllipsis from '~icons/lucide/rectangle-ellipsis';
	import LucideLanguages from '~icons/lucide/languages';
	import LucideGlobe from '~icons/lucide/globe';
	import { Button } from '$components/ui/button';
	import { getTranslate, T } from '@tolgee/svelte';

	const { t } = getTranslate();

	$title = $t('account_settings.account_settings');

	const { data }: { data: PageData } = $props();
</script>

<div class="mx-3 mb-16">
	<div class="mt-3 mb-5 grid grid-cols-6">
		<div class="flex items-center">
			<Button href="/" variant="outline" size="icon">
				<LucideChevronLeft class="size-6" />
			</Button>
		</div>
		<h1 class="col-span-4 text-center text-3xl font-bold">
			<T keyName="account_settings.account_settings" />
		</h1>
		<div></div>
	</div>

	<div class="grid gap-6 md:grid-cols-2">
		<!-- Change language -->
		<Card.Root>
			<Card.Header>
				<Card.Title>
					<LucideLanguages class="mb-5 size-10" />
					<T keyName="account_settings.change_language" />
				</Card.Title>
				<Card.Description>
					<T keyName="account_settings.change_language_description" />
				</Card.Description>
			</Card.Header>
			<Card.Content>
				<ChangeLanguageForm data={data.languageForm} />
			</Card.Content>
		</Card.Root>

		<!-- Change timezone -->
		<Card.Root>
			<Card.Header>
				<Card.Title>
					<LucideGlobe class="mb-5 size-10" />
					<T keyName="account_settings.change_timezone" />
				</Card.Title>
				<Card.Description>
					<T keyName="account_settings.change_timezone_description" />
				</Card.Description>
			</Card.Header>
			<Card.Content>
				<ChangeTimezoneForm data={data.timezoneForm} />
			</Card.Content>
		</Card.Root>

		<div class="flex justify-center md:col-span-2">
			<!-- Change password -->
			<Card.Root class="w-full md:w-1/2">
				<Card.Header>
					<Card.Title>
						<LucideRectangleEllipsis class="mb-5 size-10" />
						<T keyName="account_settings.change_password" />
					</Card.Title>
				</Card.Header>
				<Card.Content>
					<ChangePasswordForm data={data.passwordForm} />
				</Card.Content>
			</Card.Root>
		</div>
	</div>
</div>
