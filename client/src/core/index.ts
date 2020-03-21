export interface Mailbox {
	messages: string[]
}

export interface Beer {
	id: string
	sortOrder: number
	name: string
	beerType: string
	abv: string
	ibu: string
	servingSize: string
	cost: string
	breweryName: string
	breweryCity: string
	breweryState: string
	kegId: string
	kegSize: number
	kegAmountLeft: number
	updatedTs: number
	createdTs: number
}

export const defaultBeer: Beer = {
	id: "",
	sortOrder: 0,
	name: "",
	beerType: "",
	abv: "",
	ibu: "",
	servingSize: "",
	cost: "",
	breweryName: "",
	breweryCity: "",
	breweryState: "",
	kegId: "",
	kegSize: 0,
	kegAmountLeft: 0,
	updatedTs: 0,
	createdTs: 0,
}
