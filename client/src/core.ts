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
    sortOrder: number
    name: string
    beerType: string
    abv: string
    ibu: string
    servingSize: string
    cost: string
    breweryName: string
    breweryCity: string
    breweryState: string
    breweryImgUrl: string
    kegId: string
    kegSize: number
    kegAmountLeft: number
    updatedTs: number
    createdTs: number
}

export const defaultBeer = {
    id: "",
    sortOrder: 0,
    name: "",
    beerType: "",
    abv: "",
    ibu: "",
    servingSize: "",
    cost: "",
    breweryName: "",
    breweryCity: "",
    breweryState: "",
    breweryImgUrl: "",
    kegId: "",
    kegSize: 0,
    kegAmountLeft: 0,
    updatedTs: 0,
    createdTs: 0,
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
