<script>
    import { beerStore } from "./stores/beer"
    import { constants } from "./constants/index"
    import { fetchBeerList } from "./utils/beer_utils"
    import Beer from "./components/beer.svelte"

    const { beer: { TAP_LIST } } = constants

    const getBeers = () => {
        fetchBeerList()
    }

    const reset = () => {
        beerStore.reset()
    }
</script>

<div class="app-container">
    <div class="app-title">
        <h1>
            {TAP_LIST}
            <button on:click={getBeers}>push button</button>
            <button on:click={reset}>reset button</button>
        </h1>
    </div>

    <div class="beers-container">
        {#each $beerStore.beers as beer}
            <Beer beer={beer} />
        {/each}
    </div>
</div>

<style>
    h1 {
        color: tomato;
    }

    .beers-container {
        display: grid;
        grid-template-columns: 50% 50%;
    }
</style>
