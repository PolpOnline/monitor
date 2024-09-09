import { z } from 'zod';

export const formSchema = z.object({
	name: z.string(),
	frequency: z.coerce.number().int().positive().default(30),
	starts_at: z.string().datetime()
});

export type FormSchema = typeof formSchema;
