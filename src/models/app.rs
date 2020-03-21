use serde_derive::{Deserialize, Serialize};
//  all app state models live in this file

#[derive(Serialize, Deserialize, Debug)]
pub struct Beer {
    pub id: String,
    pub sort_order: i32,
    pub name: String,
    pub beer_type: String,
    pub abv: String,
    pub ibu: String,
    pub serving_size: String,
    pub cost: String,
    pub brewery_name: String,
    pub brewery_city: String,
    pub brewery_state: String,
    pub keg_id: String,
    pub keg_size: i32,
    pub keg_amount_left: i32,
    pub updated_ts: i32,
    pub created_ts: i32,
}

pub enum BeerFilters {
    Id,
    SortOrder,
    Name,
    BeerType,
    Abv,
    Ibu,
    ServingSize,
    Cost,
    BreweryName,
    BreweryCity,
    BreweryState,
    KegId,
    KegSize,
    KegAmountLeft,
    UpdatedTs,
    CreatedTs,
    Error,
}
