<script>
    import { beerStore } from "./stores/beer"
    import { beerConstants } from "./constants/beer"
    import { fetchBeerList } from "./utils/beer_utils"
    import Beer from "./components/beer.svelte"

    const { TAP_LIST } = beerConstants

    const refreshBeerList = () => {
        clearList()
        fetchBeerList()
    }

    const clearList = () => {
        beerStore.reset()
    }
</script>

<div id="app-container">
    <div class="app-title">
        <h1>
            {TAP_LIST}
            <button on:click={refreshBeerList}>fetch</button>
            <button on:click={clearList}>clear all</button>
        </h1>
    </div>

    <div class="beers-container">
        {#each $beerStore.beers as beer}
            <Beer beer={beer} />
        {/each}
    </div>
</div>

<style>
    #app-container {
        background: rgb(68,84,98);
    }

    h1 {
        color: tomato;
    }

    .beers-container {
        display: grid;
        grid-template-columns: 50% 50%;
    }
</style>
