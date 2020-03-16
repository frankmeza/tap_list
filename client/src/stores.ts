import { writable } from "svelte/store"
// import { Beer } from "./core"

// interface BeerStore {
// 	beers: Beer[]
// }

const createBeerStore = () => {
    const emptyStore = { beers: [] }
	const { subscribe, set, update } = writable(emptyStore)

	const receiveNewBeerData = (newBeers) =>
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

export const beerStore = createBeerStore()
