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
	breweryImgUrl: string
	kegId: string
	kegSize: number
	kegAmountLeft: number
	updatedTs: number
	createdTs: number
}

export const defaultBeer = {
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
	breweryImgUrl: "",
	kegId: "",
	kegSize: 0,
	kegAmountLeft: 0,
	updatedTs: 0,
	createdTs: 0,
}
