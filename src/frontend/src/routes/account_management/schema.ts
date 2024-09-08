import { z } from 'zod';

export const formSchema = z
	.object({
		old_password: z.string().min(8),
		new_password: z.string().min(8),
		new_password_confirm: z.string().min(8)
	})
	.refine((data) => data.new_password === data.new_password_confirm, {
		message: "Passwords don't match",
		path: ['confirm'] // path of error
	});

export type FormSchema = typeof formSchema;
