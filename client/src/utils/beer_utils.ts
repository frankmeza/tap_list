import { Beer } from "core"
import { BASE_URL, BeerRemote } from "../remote"
import { camelCaseObject } from "./transform"
import { beerStore } from "stores"

export async function fetchBeerList(): Promise<boolean> {
	const response = await fetch(`${BASE_URL}/beers`)

	if (!response.ok) {
		const error = await response.json()
		console.error(error.message)

		return false
	}

	const beerListRemote: BeerRemote[] = await response.json()

	const beers = beerListRemote.map((beer) => {
		return camelCaseObject(beer) as Beer
	})

	beerStore.receiveBeerList(beers)
	return true
}
