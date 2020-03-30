export const calcPercentLeft = (
    kegAmountLeft: number,
    kegSize: number,
): number => {
    return Math.round(kegAmountLeft / kegSize * 100)
}

export const calcKegColor = (amountLeft: number): string => {
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
