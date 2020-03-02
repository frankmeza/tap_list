// // EXAMPLE GET INDEX
// pub fn get_people() -> String {
//     format!("SELECT id, name, ts FROM person")
// }

// // EXAMPLE GET
// pub fn get_name_id_person(id: &str) -> String {
//     format!("SELECT id, name, ts FROM person WHERE id = '{}'", id,)
// }

// // EXAMPLE POST
// pub fn create_person(id: &str, name: &str, timestamp: u64) -> String {
//     format!(
//         "INSERT INTO person (id, name, ts) VALUES ('{}', '{}', '{}')",
//         id, name, timestamp,
//     )
// }

// // EXAMPLE PUT
// pub fn update_person_by_id(id: &str, name: &str) -> String {
//     format!("UPDATE person SET name = '{}' WHERE id = '{}'", name, id,)
// }

// // EXAMPLE DELETE
// pub fn delete_person_by_id(id: &str) -> String {
//     format!("DELETE FROM person WHERE id = '{}'", id,)
// }
