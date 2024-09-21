import * as os from 'os';

export async function GET() {
	const memory = process.memoryUsage();

	const totalMemHuman = humanizeBytes(os.totalmem());
	const freeMemHuman = humanizeBytes(os.freemem());
	const heapUsedHuman = humanizeBytes(memory.heapUsed);
	const heapTotalHuman = humanizeBytes(memory.heapTotal);

	return new Response(
		JSON.stringify({
			node: process.versions.node,
			system: process.platform,
			arch: process.arch,
			memory: process.memoryUsage(),
			memoryHuman: {
				heapTotal: heapTotalHuman,
				heapUsed: heapUsedHuman
			},
			uptime: process.uptime(),
			cpu: process.cpuUsage(),
			platform: os.type,
			cores: os.cpus().length,
			model: os.cpus()[0].model,
			speed: os.cpus()[0].speed,
			totalMem: os.totalmem(),
			freeMem: os.freemem(),
			totalMemHuman,
			freeMemHuman
		})
	);
}

function humanizeBytes(bytes: number): string {
	const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
	if (bytes === 0) return '0 B';
	const i = Math.floor(Math.log(bytes) / Math.log(1024));
	return (bytes / Math.pow(1024, i)).toFixed(2) + ' ' + sizes[i];
}
