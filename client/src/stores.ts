import { writable } from "svelte/store"
import { Beer, defaultBeer } from "core"

interface BeerStore {
	beers: Beer[]
}

const createBeerStore = () => {
	const { subscribe, set, update } = writable({ beers: [] } as BeerStore)

	const receiveNewBeerData = (newBeers: Beer[]) =>
		update((currentBeers) => {
			return {
				beers: [ ...currentBeers.beers, ...newBeers ],
			}
		})

	const reset = () => {
		set({ beers: [ defaultBeer ] } as BeerStore)
	}

	return {
		subscribe,
		receiveNewBeerData,
		reset,
	}
}

export const beerStore = createBeerStore()

// import { writable } from "svelte/store"
// // import { Beer } from "./core"

// // interface BeerStore {
// // 	beers: Beer[]
// // }

// const createBeerStore = () => {
//     const emptyStore = { beers: [] }
// 	const { subscribe, set, update } = writable(emptyStore)

// 	const receiveNewBeerData = (newBeers) =>
// 		update((currentStore) => {
// 			return {
// 				beers: [ ...currentStore.beers, ...newBeers ],
// 			}
// 		})

// 	const reset = () => {
// 		set(emptyStore)
// 	}

// 	return {
// 		subscribe,
// 		receiveNewBeerData,
// 		reset,
// 	}
// }

// export const beerStore = createBeerStore()
