// EXAMPLE
export interface Person {
    id: string
    ts: number
    name: string
}

export interface Mailbox {
	messages: string[]
}

export interface Beer {
    id: string
    sort_order: number
    name: string
    beer_type: string
    abv: string
    ibu: string
    serving_size: string
    cost: string
    brewery_name: string
    brewery_city: string
    brewery_state: string
    brewery_img_url: string
    keg_id: string
    keg_size: number
    keg_amount_left: number
    updated_ts: number
    created_ts: number
}

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
