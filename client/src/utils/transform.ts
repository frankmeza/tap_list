interface CamelCaseObject {
	[key: string]: any
}

const capitalize = (word: string): string =>
	word[0].toUpperCase() + word.slice(1)

const camelCase = (snakeCaseVariable: string): string => {
	const words = snakeCaseVariable.split("_")

	const snakeCased = words.map((word, index) => {
		return index === 0 ? word : capitalize(word)
	})

	return snakeCased.join("")
}

const camelCaseObject = (obj: {}): CamelCaseObject => {
	const newObject: CamelCaseObject = {}

	for (let [ key, value ] of Object.entries(obj)) {
		newObject[camelCase(key)] = value
	}

	return newObject
}

export { camelCase, camelCaseObject }
