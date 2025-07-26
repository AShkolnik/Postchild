import {writable} from 'svelte/store';

import type { HttpResponse, HttpRequest } from '../network/http'

export const http_response = writable<HttpResponse | null>(null);
export const http_request = writable<HttpRequest| null>(null);
