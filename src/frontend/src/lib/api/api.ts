import { dev } from '$app/environment';

export const API_URL = dev ? 'http://localhost:3000' : String(import.meta.env.API_URL);

export const LIST_SIZE = 30;
