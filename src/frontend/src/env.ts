import { defineEnvVars } from '@sveltejs/kit/env';

export const variables = defineEnvVars({
	API_URL: {
		schema: (value) => value ?? '',
		description: 'Backend API URL (falls back to the public URL when unset)'
	}
});
