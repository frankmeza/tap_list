import { Beer } from "core"
import { BASE_URL, BeerRemote } from "../remote"
import { camelCaseObject } from "./transform"

export async function getBeerList(): Promise<Beer[] | null> {
	const response = await fetch(`${BASE_URL}/beers`)

	if (!response.ok) {
        const error = await response.json()
        console.error(error.message)
        return null
	}

	const beerList: BeerRemote[] = await response.json()
	return beerList.map((beer) => camelCaseObject(beer) as Beer)
}
