const BASE_URL = "http://localhost:8088"
const headers = { "Content-Type": "application/json" }

type Request = "GET" | "POST" | "PUT" | "DELETE"

interface RequestMethod {
	[key: string]: Request
}

const requestMethod: RequestMethod = {
	GET: "GET",
	POST: "POST",
	PUT: "PUT",
	DELETE: "DELETE",
}

interface AppRequest {
	body: string
	headers: {}
	method: string
}

interface RequestBody {
	id: string
	name: string
	ts: number
}

const buildRequest = (
	method: string,
	body: Partial<RequestBody>,
): AppRequest => ({
	...baseAppRequest,
	body: JSON.stringify(body),
	headers,
	method,
})

const baseAppRequest: AppRequest = {
	body: "",
	headers,
	method: requestMethod.GET,
}

export { baseAppRequest, buildRequest, requestMethod, headers, BASE_URL }
