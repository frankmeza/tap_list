const calcPercentLeft = (kegAmountLeft: number, kegSize: number): number => {
    return Math.round(kegAmountLeft / kegSize * 100)
}

const calcKegColor = (amountLeft: number): string => {
    if (amountLeft > 50) {
        return "green"
    }

    if (amountLeft > 35) {
        return "yellow"
    }

    if (amountLeft > 20) {
        return "orange"
    }

    if (amountLeft > 10) {
        return "red"
    }

    return ""
}

const formatBeerData = (
    name: string,
    beerType: string,
    breweryName: string,
): string => `${name}${beerType}${breweryName}`

const formatImgSrc = (beerData: string) =>
    `https://robohash.org/${beerData}.png` // append ?set=set5 for humans

const formatDisplayedAbv = (abv: string) => `abv: ${abv}%`

const formatBreweryInfo = (
    breweryName: string,
    breweryCity: string,
    breweryState: string,
): string => `${breweryName} in ${breweryCity}, ${breweryState}`

export {
    calcPercentLeft,
    calcKegColor,
    // string formatting
    formatBeerData,
    formatImgSrc,
    formatDisplayedAbv,
    formatBreweryInfo,
}
