<script lang="ts">
    import { defaultBeer, Beer } from "../core"
    import SortOrderDot from "./sort_order.svelte"

    export let beer: Beer = defaultBeer
    const { sortOrder } = beer
</script>

<div class="beer-container">
    <SortOrderDot sortOrder={sortOrder} />

    <p>{JSON.stringify(beer, null, 4)}</p>
</div>

<style>
    .beer-container {
        display: grid;
        grid-template-columns: 10% 90%;
        border: solid 2px black;
        border-radius: 4px;
        margin-right: 4px;
        padding: 4px;
    }
</style>
