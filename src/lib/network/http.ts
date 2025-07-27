import type { AuthType } from "./auth_type";

interface HttpResponse {
	status: string | number;
	body: string;
	headers: Record<string, string>;
}

interface HttpRequest {
	method: HttpMethod;
	url: string;
	body: string | null;
}

export const HttpMethod = {
  Get: 'GET',
  Post: 'POST',
  Put: 'PUT',
  Delete: 'DELETE',
} as const;

export type HttpMethod = typeof HttpMethod[keyof typeof HttpMethod];

export type { HttpResponse, HttpRequest };
