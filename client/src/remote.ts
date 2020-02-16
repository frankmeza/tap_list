
export const BASE_URL = "http://localhost:8088"
export const headers = { "Content-Type": "application/json" }

export type Request = "GET" | "POST" | "PUT" | "DELETE"

export interface RequestMethod {
	[key: string]: Request
}

export const requestMethod: RequestMethod = {
	GET: "GET",
	POST: "POST",
	PUT: "PUT",
	DELETE: "DELETE",
}

export interface AppRequest {
	body: string
	headers: {}
	method: string
}

export interface RequestBody {
	id: string
	name: string
	ts: number
}

export const buildRequest = (
	method: string,
	body: Partial<RequestBody>,
): AppRequest => ({
	...baseAppRequest,
	body: JSON.stringify(body),
	headers,
	method,
})

export const baseAppRequest: AppRequest = {
	body: "",
	headers,
	method: requestMethod.GET,
}
