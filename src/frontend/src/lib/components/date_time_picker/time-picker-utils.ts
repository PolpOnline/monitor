// Regular expression to check for valid hour format (01-23)
export function isValidHour(value: string) {
	return /^(0[0-9]|1[0-9]|2[0-3])$/.test(value);
}

// Regular expression to check for valid minute/second format (00-59)
export function isValidMinuteSecond(value: string) {
	return /^[0-5][0-9]$/.test(value);
}
