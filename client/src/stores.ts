import { writable } from "svelte/store"

export const count = writable({hella_lit: true})
// class Mailbox {
//     messages: string[]

// 	constructor() {
// 		this.messages = []
// 	}
// }

// const createMailBox = () => {
// 	const { subscribe, set, update } = writable(new Mailbox())

// 	const handleAddMsg = (newMsg: string) =>
// 		update((mailbox) => {
//             return { messages: [...mailbox.messages, newMsg] }
//         })

//     return {
// 		subscribe,
// 		addMsg: (newMsg: string) => handleAddMsg(newMsg),
// 		// getMsg: () => update((n) => n - 1),
// 		reset: () => set(new Mailbox()),
// 	}
// }

// export const mailboxStore = createMailBox()

