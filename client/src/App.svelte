<script lang="ts">
    import { Beer } from "core"
    import { constants } from "./constants/index"
    import { getBeerList } from "./utils/beer_utils"
    import BeerCpx from "./components/beer.svelte"

    const { beer: { TAP_LIST } } = constants

    let beers = []

    const getBeers = async () => {
        const response = await getBeerList()

        if (response) {
            beers = response
        }
    }
</script>

<div class="app-container">
    <div class="app-title">
        <h1>
            {TAP_LIST}
            <button on:click={getBeers}>push button</button>
        </h1>

        <div class="beers">
            <BeerCpx beer={beers[0]} />
            <BeerCpx beer={beers[0]} />
        </div>
    </div>
</div>

<style>
    h1 {
        color: tomato;
    }

    .beers {
        display: grid;
        grid-template-columns: 50% 50%;
    }
</style>
