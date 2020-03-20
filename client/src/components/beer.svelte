<script>
    import { defaultBeer } from "../core"
    // import Keg from "../assets/images/keg_image.svg"

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
    // TODO ROBOHASH https://robohash.org/ for image url -> create a hash from name, beerType, breweryName
</script>

<!-- <pre>{JSON.stringify(beer, null, 4)}</pre> -->
<div class="beer-container">
    <div class="section">
        {sortOrder}
    </div>

    <div class="section">
        {breweryImgUrl}
    </div>

    <div class="section">
        <p>{breweryName} {name}</p>
        <p>{beerType}</p>
        <p>{breweryCity}, {breweryState} ABV {abv}%</p>
    </div>

    <div class="section">
        <p>KEG AMOUNT LEFT: {percentLeft}%</p>
    </div>
</div>

<style>
    .beer-container {
        display: grid;
        grid-template-columns: 15% 15% 55% 15%;

        margin-right: 4px;
        padding: 4px;

        background: rgb(34,193,195);
        background: linear-gradient(0deg, rgba(34,193,195,1) 0%, rgba(253,187,45,1) 100%);
    }

    .section {
        padding: 1em;
    }
</style>
