import type { PageLoad } from './$types';

export const load: PageLoad = ({ locals }) => {
	return {
		loginStatus: locals.loginStatus
	};
};
