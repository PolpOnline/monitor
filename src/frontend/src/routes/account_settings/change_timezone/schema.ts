import { z } from 'zod';

export const formSchema = z.object({
	timezone: z.string()
});

export type FormSchema = typeof formSchema;
