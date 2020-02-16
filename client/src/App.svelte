<script lang="ts">
    import { Beer } from "core"
    const BEER_LIST = "Beer Tap List"
    import { getBeerList } from "./utils/beer_utils"

    let beers: Beer[] = []

    const getBeers = async () => {
        beers = await getBeerList()
        return beers
    }
</script>


<h1>
    {BEER_LIST}
</h1>

<button on:click={getBeers}>
    push button
</button>

<pre>{JSON.stringify(beers, null, 4)}</pre>

<style>
    h1 {
        color: tomato;
    }
</style>
