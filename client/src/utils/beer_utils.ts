import { Beer } from "core"
import { BASE_URL, BeerRemote } from "../core/remote"
import { camelCaseObject } from "./transform"
import { beerStore } from "../stores/beer"

export async function fetchBeerList(): Promise<boolean> {
    const response = await fetch(`${BASE_URL}/beers`)

    if (!response.ok) {
        const error = await response.json()
        console.error(error.message)

        return false
    }

    const beerListRemote: BeerRemote[] = await response.json()

    const beers = beerListRemote
        .map(beer => camelCaseObject(beer) as Beer)
        .sort((a, b) => (a.sortOrder > b.sortOrder ? 1 : -1))

    beerStore.receiveBeerList(beers)
    return true
}
