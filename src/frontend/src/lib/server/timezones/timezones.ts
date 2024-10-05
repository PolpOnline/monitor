import { getTimeZones } from '@vvo/tzdb';

const tzDB = getTimeZones();

export const timezones = tzDB.map((timezone) => {
	return {
		label: timezone.name,
		value: timezone.name
	};
});
