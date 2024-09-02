import type { AllSystemsData, Instant, Status, SystemData } from '$lib/types/items';
import type { PageLoad } from './$types';

// disable ssr for now, as it interferes with the random status generation
export const ssr = false;

function generateRandomStatus(): Status {
	const statuses: Status[] = ['ok', 'warning', 'error'];
	return statuses[Math.floor(Math.random() * statuses.length)];
}

function generateSystemData(name: string, frequency: number): SystemData {
	const instants: Instant[] = [];
	const startTime = new Date(Date.now());

	for (let i = 0; i < 30; i++) {
		instants.push({
			status: generateRandomStatus(),
			timestamp: new Date(startTime.getTime() - i * frequency * 60 * 1000)
		});
	}

	instants.reverse();

	return {
		name,
		instants,
		frequency
	};
}

const exampleSystemData1 = generateSystemData('Test System 1', 30); // 30 minutes
console.log(exampleSystemData1);
const exampleSystemData2 = generateSystemData('Test System 2', 60); // 1 hour
const exampleSystemData3 = generateSystemData('Test System 3', 180); // 3 hours

export const load: PageLoad = () => {
	const allSystemsExampleData: AllSystemsData = {
		systems: [exampleSystemData1, exampleSystemData2, exampleSystemData3]
	};

	return allSystemsExampleData;
};
