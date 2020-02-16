import {
    buildRequest,
    requestMethod as r,
    RequestBody,
    BASE_URL
} from "../remote"
import { Person } from "../core"

// TODO add error handling
export async function getPeople(): Promise<Person[]> {
	const response: Response = await fetch(`${BASE_URL}/people`)
	const people: Person[] = await response.json()

	return people
}

// TODO add error handling
export async function getPersonById(id: string): Promise<Person> {
	const reqBody: Partial<RequestBody> = { id }

	const request = buildRequest(r.POST, reqBody)
	const response = await fetch(`${BASE_URL}/people/${id}`, request)
	const person: Person = await response.json()

	return person
}

const statusReturnHelper = (rs: Response): string => {
	if (rs.ok) {
		console.log("operation successful 👍")
		return "ok"
	} else {
		console.log(`very virus: ${rs}`)
		return rs.statusText
	}
}

export async function createPerson(name: string): Promise<string> {
	const reqBody: Partial<RequestBody> = { name }

	const request = buildRequest(r.POST, reqBody)
	const response = await fetch(`${BASE_URL}/people`, request)

	return statusReturnHelper(response)
}

export async function updatePersonById(person: Person): Promise<string> {
	const { id, name } = person
	const reqBody: Partial<RequestBody> = { id, name, ts: Date.now() }

	const request = buildRequest(r.PUT, reqBody)
	const response = await fetch(`${BASE_URL}/people`, request)

	return statusReturnHelper(response)
}

export async function deletePersonById(id: string): Promise<string> {
	const reqBody: Partial<RequestBody> = { id }

	const request = buildRequest("DELETE", reqBody)
	const response = await fetch(`${BASE_URL}/people`, request)

	return statusReturnHelper(response)
}
