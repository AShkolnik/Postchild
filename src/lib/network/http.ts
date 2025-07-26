import type { AuthType } from "./auth_type";

interface Cookie {
	name: string;
	value: string;
	domain: string;
	path: string;
	http_only: boolean;
	secure: boolean;
}

interface Header {
	date: Date;
	content_type: string;
	transfer_encoding: string;
	connection: string;
	server: string;
}

interface HttpResponse {
	body: string;
	cookies: Cookie;
	headers: Header;
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

export type { Cookie, Header, HttpResponse, HttpRequest };
