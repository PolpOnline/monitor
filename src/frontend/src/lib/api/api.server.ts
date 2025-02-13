import createClient from 'openapi-fetch';
import type { paths } from '$lib/api/schema';
import { env } from '$env/dynamic/private';
import { PUBLIC_API_URL } from './public-api';

export const API_URL = env.API_URL || PUBLIC_API_URL;

export const client = createClient<paths>({ baseUrl: API_URL });
