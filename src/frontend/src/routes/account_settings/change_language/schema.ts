import { z } from 'zod';

export const formSchema = z.object({
	language: z.string()
});

export type FormSchema = typeof formSchema;
