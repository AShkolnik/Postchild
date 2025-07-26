import {writable} from 'svelte/store';

import type { HttpResponse } from '../network/http'

export const http_response = writable<HttpResponse | null>(null);
