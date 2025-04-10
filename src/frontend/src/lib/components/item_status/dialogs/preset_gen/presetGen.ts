import { PUBLIC_API_URL } from '$lib/api/public-api';
import type { components } from '$lib/api/schema';
import dedent from 'dedent';

export const presets = [
	{
		value: 'mikrotik',
		label: 'MikroTik RouterOS'
	}
];

export function getPresets(
	targetSystemData: components['schemas']['SystemData']
): Record<string, string> {
	return {
		mikrotik: getMikrotikPreset(targetSystemData)
	};
}

function getMikrotikPreset(targetSystemData: components['schemas']['SystemData']): string {
	const startsAtDateTime = new Date(targetSystemData.starts_at);

	function getStartsAtDate(dateTime: Date) {
		return dateTime
			.toLocaleDateString('en-US', { year: 'numeric', month: 'short', day: '2-digit' })
			.replace(',', '')
			.replaceAll(' ', '/')
			.toLowerCase();
	} // Format: mmm/dd/yyyy (e.g. jan/01/2022)

	function getStartsAtTime(dateTime: Date) {
		return dateTime.toLocaleTimeString('en-GB', { hour12: false });
	} // Format: 00:00:00

	const startsAtDate = getStartsAtDate(startsAtDateTime);
	const startsAtTime = getStartsAtTime(startsAtDateTime);

	const frequencyHours = Math.floor(targetSystemData.frequency / 60);
	const frequencyMinutes = targetSystemData.frequency % 60;

	return dedent`/system scheduler add name="ping_status" start-date="${startsAtDate}" start-time="${startsAtTime}" interval="${frequencyHours}:${frequencyMinutes}:00"
					on-event="/tool fetch url=\"${PUBLIC_API_URL}/ping_status/${targetSystemData?.id}\"
					mode=https http-method=post output=none"`;
}
