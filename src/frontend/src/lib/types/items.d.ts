export type Status = 'up' | 'down';

export type Instant = {
	status: Status;
	timestamp: Date;
};

export type SystemData = {
	name: string;
	instants: Instant[];
	// Frequency in minutes
	frequency: number;
};

export type AllSystemsData = {
	systems: SystemData[];
};
