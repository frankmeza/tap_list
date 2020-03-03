// use crate::{
//     models::{Beer, BeerFilters},
//     queries::beer_utils,
// };

pub fn get_beer_list() -> String {
    format!("SELECT * FROM beers;")
}

// pub fn get_beers_filtered_by(filter: BeerFilters, value: &str) -> String {
//     let filter_item = beer_utils::generate_filter_string(filter);
//     format!("SELECT * FROM beers WHERE {} = '{}';", filter_item, value)
// }

// pub fn create_beer(beer: Beer) -> String {
//     beer_utils::generate_create_query_string(beer)
// }

// pub fn update_beer_by_id(id: &str, updated_beer: Beer) -> String {
//     let found_beer = get_beers_filtered_by(BeerFilters::Id, id);

// }