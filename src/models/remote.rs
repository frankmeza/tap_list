use serde_derive::{Deserialize, Serialize};
// all client request models live in this file

// EXAMPLE
// #[derive(Serialize, Deserialize, Debug)]
// pub struct Id {
//     pub id: String,
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct PersonReq {
//     pub name: String,
// }

//////////

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BeerRequest {
    pub name: String,
    pub beer_type: String,
    pub abv: String,
    pub ibu: String,
    pub serving_size: String,
    pub cost: String,
    pub brewery_name: String,
    pub brewery_city: String,
    pub brewery_state: String,
    pub brewery_img_url: String,
    pub keg_id: String,
    pub keg_size: i32,
    pub keg_amount_left: i32,
    pub updated_ts: i32,
    pub created_ts: i32,
}
