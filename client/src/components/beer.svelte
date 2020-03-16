<script>
    import { defaultBeer } from "../core"

    export let beer = defaultBeer

    const {
        id = "",
        sortOrder = 0,
        name = "",
        beerType = "",
        abv = "",
        ibu = "",
        servingSize = "",
        cost = "",
        breweryName = "",
        breweryCity = "",
        breweryState = "",
        breweryImgUrl = "",
        kegId = "",
        kegSize = 0,
        kegAmountLeft = 0,
        updatedTs = 0,
        createdTs = 0,
    } = beer

    $: percentLeft = Math.round(kegSize / kegAmountLeft)
</script>

<!-- <pre>{JSON.stringify(beer, null, 4)}</pre> -->
<div class="beer-container">
    <div class="temp-border">
        {sortOrder}
    </div>

    <div class="temp-border">
        {breweryImgUrl}
    </div>

    <div class="temp-border">
        <p>{breweryName} {name}</p>
        <p>{beerType}</p>

        <p>{breweryCity}, {breweryState} ABV {abv}%</p>
    </div>

    <div class="temp-border">
        <p>KEG AMOUNT LEFT: {percentLeft}%</p>
    </div>
</div>

<style>
    .beer-container {
        display: grid;
        grid-template-columns: 15% 15% 55% 15%;
        border: solid 2px black;
        border-radius: 4px;
        margin-right: 4px;
        padding: 4px;
    }

    .temp-border {
        border: solid 2px black;
    }
</style>
