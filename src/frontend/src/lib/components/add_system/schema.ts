import { z } from 'zod';

export const formSchema = z.object({
	name: z.string().trim().min(1, { message: 'Name is required' }),
	frequency: z.coerce.number().int().positive().default(240),
	starts_at: z.string().datetime(),
	down_after: z.coerce.number().int().positive().default(240),
	visibility: z.enum(['public', 'private']).default('private')
});

export type FormSchema = typeof formSchema;
