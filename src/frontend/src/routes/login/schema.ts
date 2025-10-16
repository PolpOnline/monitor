import { z } from 'zod';

export const formSchema = z.object({
	email: z.email().trim().toLowerCase(),
	password: z.string().min(8)
});

export type FormSchema = typeof formSchema;
