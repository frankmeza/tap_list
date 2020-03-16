import { writable } from "svelte/store"
import { Beer } from "../core"

interface BeerStore {
	beers: Beer[]
}

const emptyStore: BeerStore = { beers: [] }

export const createBeerStore = () => {
	const { subscribe, set, update } = writable(emptyStore)

	const receiveNewBeerData = (newBeers: Beer[]) =>
		update((currentStore) => {
			return {
				beers: [ ...currentStore.beers, ...newBeers ],
			}
		})

	const reset = () => {
		set(emptyStore)
	}

	return {
		subscribe,
		receiveNewBeerData,
		reset,
	}
}
