<script>
    import { beerStore } from "./stores/beer"
    import { ws } from "./ws_client"
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

    const uiButtons = [{
        onClick: refreshBeerList,
        text: "fetch list",
    }, {
        onClick: clearList,
        text: "clear all",
    }]
</script>

<div id="app-container">
    <div class="app-title">
        <h1>
            {TAP_LIST}
        </h1>
    </div>

    {#each uiButtons as uiButton}
        <div
            class="async-buttons"
            on:click={uiButton.onClick}>
            {uiButton.text}
        </div>
    {/each}

    <div class="beers-container">
        {#each $beerStore.beers as beer}
            <Beer beer={beer} />
        {/each}
    </div>
</div>

<style>
    #app-container {
        background: rgb(68,84,98);
        min-height: 7em;
    }

    .app-title {
        color: gold;
        padding: 0 0 0 0.8em;
    }

    .beers-container {
        display: grid;
        grid-template-columns: 50% 50%;
        margin-top: 20px;
    }

    .async-buttons {
        border: 2px solid #ccc;
        border-radius: 0.25em;
        color: #ccc;
        display: inline;
        padding: 0.5em;
        margin: 0.75em;
    }
</style>
