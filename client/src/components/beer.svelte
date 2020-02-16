<script lang="ts">
    import { defaultBeer } from "../core"
    export let beer = defaultBeer

    const { sortOrder } = beer
</script>

<div class="beer-container">
    <div class="sort-order-dot">
        {sortOrder}
    </div>
    <p>{JSON.stringify(beer, null, 4)}</p>
</div>

<style>
    .beer-container {
        /* display: grid;
        grid-template-columns: 50% 50%; */
        border: solid 2px black;
        border-radius: 4px;
        margin-right: 4px;
        padding: 4px;
    }

    .sort-order-dot {
        border: solid 2px black;
        border-radius: 25%;
        margin-right: 4px;
        height: 50%;
        width: 10%;
    }
</style>