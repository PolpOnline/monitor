import { dev } from '$app/environment';

export const PUBLIC_API_URL = dev ? 'http://localhost:3000' : 'https://monitor-api.polp.online';

export const LIST_SIZE = 30;
