const ADD_RECORD = (name: string) => {
	return name.length > 0 ? `add record for ${name}` : "add record"
}

const UPDATE_RECORD = (id: string) => {
	return id.length > 0 ? `update record for ${id}` : "update record"
}

const DELETE_RECORD = (id: string) => {
	return id.length > 0 ? `delete record for ${id}` : "delete record"
}

const GET_RECORDS = "GET records"
const RESET = "reset client"

export const example = {
	ADD_RECORD,
	UPDATE_RECORD,
	DELETE_RECORD,
	GET_RECORDS,
	RESET,
}
