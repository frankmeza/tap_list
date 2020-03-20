import { writable } from "svelte/store"
import { Beer, defaultBeer } from "core"

interface BeerStore {
    beers: Beer[]
}

const emptyStore: BeerStore = { beers: [] }

const createBeerStore = () => {
    const { subscribe, set, update } = writable(emptyStore)

    // ...currentStore.beers can be put into the new array, as needed
    const receiveBeerList = (newBeers: Beer[]) =>
        update(currentStore => ({ beers: [ ...newBeers ] }))

    const reset = () => set(emptyStore)

    return {
        subscribe,
        receiveBeerList,
        reset,
    }
}

export const beerStore = createBeerStore()
