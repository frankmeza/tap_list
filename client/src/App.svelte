<script lang="ts">
    import { Beer } from "core"
    import { constants } from "./constants/index"
    import { getBeerList } from "./utils/beer_utils"
    import BeerCpx from "./components/beer.svelte"

    const { TAP_LIST } = constants.beer

    let beers: Beer[] = []

    const getBeers = async () => {
        beers = await getBeerList()
        return beers
    }
</script>

<h1>{TAP_LIST}

<button on:click={getBeers}>push button</button>

</h1>
<div class="beers">
    <BeerCpx beer={beers[0]} />
    <BeerCpx beer={beers[0]} />
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
