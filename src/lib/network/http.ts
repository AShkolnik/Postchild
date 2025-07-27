import type { AuthType } from "./auth_type";

interface HttpResponse {
	body: string;
	headers: Record<string, string>;
}

interface HttpRequest {
	method: HttpMethod;
	url: string;
	authorization: AuthType;
	headers?: Record<string, string>;
	body?: string | object;
	params?: Record<'key' | 'value', string>;
}

export const HttpMethod = {
  Get: 'GET',
  Post: 'POST',
  Put: 'PUT',
  Delete: 'DELETE',
  Patch: 'Patch',
  Options: 'OPTIONS',
  HEAD: 'HEAD',
} as const;

export type HttpMethod = typeof HttpMethod[keyof typeof HttpMethod];

export type { HttpResponse, HttpRequest };
