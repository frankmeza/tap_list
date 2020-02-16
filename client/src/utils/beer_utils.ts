import { Beer } from "core"
import { BASE_URL, BeerRemote } from "../remote"
import { camelCaseObject } from "./transform"

export async function getBeerList(): Promise<Beer[]> {
    const response = await fetch(`${BASE_URL}/beers`)
    const beerList: BeerRemote[] = await response.json()

    return beerList.map(beer => camelCaseObject(beer) as Beer)
}
