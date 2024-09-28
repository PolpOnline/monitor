import { z } from 'zod';

export const formSchema = z.object({
	name: z.string().min(1, { message: 'Name is required' }),
	frequency: z.coerce.number().int().positive().default(30),
	starts_at: z.string().datetime(),
	down_after: z.coerce.number().int().positive().default(240),
	visibility: z.enum(['public', 'private']).default('private')
});

export type FormSchema = typeof formSchema;
