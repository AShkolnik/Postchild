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

export type { Cookie, Header, HttpResponse };
