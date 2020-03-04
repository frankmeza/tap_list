use crate::models::{BeerFilters};

// helps set the type of filter for the query
pub fn generate_filter_string(filter: BeerFilters) -> String {
    match filter {
        BeerFilters::Id => String::from("id"),
        BeerFilters::SortOrder => String::from("sort_order"),
        BeerFilters::Name => String::from("name"),
        BeerFilters::BeerType => String::from("beer_type"),
        BeerFilters::Abv => String::from("abv"),
        BeerFilters::Ibu => String::from("ibu"),
        BeerFilters::ServingSize => String::from("serving_size"),
        BeerFilters::Cost => String::from("cost"),
        BeerFilters::BreweryName => String::from("brewery_name"),
        BeerFilters::BreweryCity => String::from("brewery_city"),
        BeerFilters::BreweryState => String::from("brewery_state"),
        BeerFilters::KegId => String::from("keg_id"),
        BeerFilters::KegSize => String::from("keg_size"),
        BeerFilters::KegAmountLeft => String::from("keg_amount_left"),
        BeerFilters::UpdatedTs => String::from("updated_ts"),
        BeerFilters::CreatedTs => String::from("created_ts"),
    }
}

// pub fn generate_create_query_string(beer: Beer) -> String {
//     let mut insert_statement = format!(
//         "INSERT INTO beers (\
//         id, \
//         sort_order, \
//         name, \
//         type, \
//         abv, \
//         ibu, \
//         serving_size, \
//         cost, \
//         brewery_name, \
//         brewery_city, \
//         brewery_state, \
//         brewery_img_url, \
//         keg_id, \
//         keg_size, \
//         keg_amount_left, \
//         updated_ts, \
//         created_ts) ",
//     );

//     let values = format!(
//         "VALUES ('{}', \
//         '{}', '{}', '{}', '{}',
//         '{}', '{}', '{}', '{}',
//         '{}', '{}', '{}', '{}',
//         '{}', '{}', '{}', '{}'",
//         beer.id,
//         beer.sort_order,
//         beer.name,
//         beer.beer_type,
//         beer.abv,
//         beer.ibu,
//         beer.serving_size,
//         beer.cost,
//         beer.brewery_name,
//         beer.brewery_city,
//         beer.brewery_state,
//         beer.brewery_img_url,
//         beer.keg_id,
//         beer.keg_size,
//         beer.keg_amount_left,
//         beer.updated_ts,
//         beer.created_ts,
//     );

//     insert_statement.push_str(&values);
//     insert_statement
// }
