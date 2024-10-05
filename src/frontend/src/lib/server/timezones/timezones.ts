import { getTimeZones } from '@vvo/tzdb';

const tzDB = getTimeZones();

export const timezones = tzDB.map((timezone) => {
	const offsetMinutes = timezone.currentTimeOffsetInMinutes;
	const offsetHours = offsetMinutes / 60;
	const offsetFormatted = offsetHours > 0 ? '+' + offsetHours : String(offsetHours);

	return {
		label: timezone.name + ' (UTC' + offsetFormatted + ':00)',
		value: timezone.name
	};
});
