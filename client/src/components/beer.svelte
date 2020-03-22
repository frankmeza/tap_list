<script>
    import { defaultBeer } from "../core"
    // import Keg from "../assets/images/keg_image.svg"

    export let beer
    export let render = false

    let {
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
        kegId = "",
        kegSize = 0,
        kegAmountLeft = 0,
        updatedTs = 0,
        createdTs = 0,
    } = beer

    $: percentLeft = Math.round(kegSize / kegAmountLeft)

    let beerData = `${name}${beerType}${breweryName}`
    let imgSrc = `https://robohash.org/${beerData}.png` // append ?set=set5 for humans

    let displayedAbv = `abv: ${abv}%`
    let breweryInfo = `${breweryName} in ${breweryCity}, ${breweryState}`
</script>

<div class="beer-container">
    <div class="section">
        <p class="tap-number">{sortOrder}</p>
    </div>

    <div class="section">
        <img class="robohash" src={imgSrc} alt="robohash image" />
    </div>

    <div class="section brewery-info">
        <p class="beer-name">{name}</p>

        <p class="beer-type-abv">
            <span class="beer-type">{beerType}</span>
            <span class="beer-abv">{displayedAbv}</span>
            <span class="beer-cost">${cost}</span>
        </p>

        <p class="brewery-location">{breweryInfo}</p>
    </div>

    <div class="section">
        <p>KEG AMOUNT LEFT: {percentLeft}%</p>
    </div>
</div>

<style>
    .beer-container {
        background: rgb(68,84,98);
        border: 2px solid #ddd;
        border-radius: 1em;
        display: grid;
        grid-template-columns: 10% 15% 55% 20%;
        margin: 0.25em;
        padding: 0.25em;
    }

    .section {
        padding: 0.5em;
        max-height: 8.5em;
    }

    .tap-number {
        color: #ddd;
        font-family: 'Courier New', Courier, monospace;
        font-size: 5em;
        font-weight: bold;
        margin: 0.15em 0 0 0.1em;
        padding: 0.1em 0 0 0.25em;
    }

    .robohash {
        max-height: 7em;
    }

    .beer-cost,
    .beer-type,
    .beer-name {
        color: #efe;
        font-family: 'Fira Code', 'Courier New', Courier, monospace;
        font-weight: bold;
        font-size: 1.25em;
        margin: 0.5em 0;
    }

    .beer-name {
        font-size: 1.75em;
    }

    .beer-name,
    .beer-type {
        font-style: italic;
    }

    .beer-abv,
    .beer-cost,
    .beer-type {
        margin-right: 0.5em;
    }

    .beer-abv {
        background-color: chartreuse;
        border-radius: 0.5em;
        color: #333;
        font-size: 1em;
        font-style: bold;
        padding: 0.25em 0.5em;
    }

    .brewery-location {
        border-radius: 0.5em;
        color: chartreuse;
        font-family: 'Courier New', Courier, monospace;
        font-size: 1em;
        font-style: none;
    }

    .beer-cost {
        font-style: none;
        color: gold;
    }
</style>
