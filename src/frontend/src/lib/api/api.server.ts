import createClient from 'openapi-fetch';
import type { paths } from '$lib/api/schema';
import { API_URL as PRIVATE_API_URL } from '$app/env/private';
import { PUBLIC_API_URL } from './public-api';

export const API_URL = PRIVATE_API_URL || PUBLIC_API_URL;

export const client = createClient<paths>({ baseUrl: API_URL });
