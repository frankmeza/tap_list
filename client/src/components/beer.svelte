<script>
    import { defaultBeer } from "../core"
    import KegSvg from "../images/keg.svg"
    import {
        calcKegColor,
        calcPercentLeft,
        formatBeerData,
        formatImgSrc,
        formatDisplayedAbv,
        formatBreweryInfo,
    } from "./ui_utils/beer"

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

    let percentLeft = calcPercentLeft(kegAmountLeft, kegSize)
    let kegColor = calcKegColor(percentLeft)

    let beerData = formatBeerData(name, beerType, breweryName)
    let imgSrc = formatImgSrc(beerData)
    let displayedAbv = formatDisplayedAbv(abv)

    let breweryInfo = formatBreweryInfo(
        breweryName,
        breweryCity,
        breweryState,
    )
</script>

<div class="beer-container">
    <div class="section beer-tap">
        <p class="tap-number">{sortOrder}</p>
    </div>

    <div class="section robohash-image">
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

    <div class="section keg-container">
        <div class={`keg keg-${kegColor}`}>
            <KegSvg />
        </div>
    </div>
</div>

<style>
    .beer-container {
        background: rgb(68,84,98);
        border: 2px solid #ddd;
        border-radius: 1rem;
        display: grid;
        grid-template-columns: 10% 15% 55% 20%;
        margin: 0.25rem;
        padding: 0.25rem;
    }

    .section {
        padding: 0.5rem;
        max-height: 8.5rem;
    }

    .tap-number {
        color: #ddd;
        font-family: 'Courier New', Courier, monospace;
        font-size: 5rem;
        font-weight: bold;
        margin: 0;
        padding: 2rem 0 0 1.25rem;
    }

    .robohash {
        max-height: 7rem;
    }

    .beer-cost,
    .beer-type,
    .beer-name {
        color: #efe;
        font-family: 'Fira Code', 'Courier New', Courier, monospace;
        font-weight: bold;
        font-size: 1.25rem;
        margin: 0.5rem 0;
    }

    .beer-name {
        font-size: 1.75rem;
    }

    .beer-name,
    .beer-type {
        font-style: italic;
    }

    .beer-abv,
    .beer-cost,
    .beer-type {
        margin-right: 0.5rem;
    }

    .beer-abv {
        background-color: chartreuse;
        border-radius: 0.5rem;
        color: #333;
        font-size: 1rem;
        font-style: bold;
        padding: 0.25rem 0.5rem;
    }

    .brewery-location {
        border-radius: 0.5rem;
        color: chartreuse;
        font-family: 'Courier New', Courier, monospace;
        font-size: 1rem;
        font-style: none;
    }

    .beer-cost {
        font-style: none;
        color: gold;
    }

    .keg-container {
        margin: 3rem 0rem 1rem 4rem;
    }

    .keg {
        width: 3.4rem;
        height: 4rem;
    }

    .keg-neutral {
        background-color: #efe;
    }

    .keg-green {
        background-color: chartreuse;
    }

    .keg-yellow {
        background-color: gold;
    }

    .keg-orange {
        background-color: orange;
    }

    .keg-red {
        background-color: red;
    }
</style>
