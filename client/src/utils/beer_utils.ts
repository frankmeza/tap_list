import { Beer } from "core"
import { requestMethod as r, headers, BASE_URL } from "./index"

export async function getBeerList(): Promise<Beer[]> {
    const response = await fetch(`${BASE_URL}/beers`)
    const beerList: Beer[] = await response.json()

    return beerList
}