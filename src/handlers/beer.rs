use crate::{
    get_async_connection,
    models::{Beer, BeerFilters},
    queries::{get_beer_list, get_beers_filtered_by},
    handlers::utils::gather_beer_list,
};
use tokio_postgres::Error;

pub async fn handle_beer_list_filtered_by(
    filter: BeerFilters,
    value: &str,
) -> Result<Vec<Beer>, Error> {
    let statement = get_beers_filtered_by(filter, value);
    let rows_future = get_async_connection(statement);

    match rows_future.await {
        Err(tokio_error) => return Err(tokio_error),
        Ok(rows) => {
            let empty_list = Vec::new();
            return gather_beer_list(rows, empty_list);
        }
    }
}

pub async fn handle_beer_list() -> Result<Vec<Beer>, Error> {
    let statement = get_beer_list();
    let rows_future = get_async_connection(statement);
    let beer_list_result = rows_future;

    match beer_list_result.await {
        Err(tokio_error) => return Err(tokio_error),
        Ok(rows) => {
            let empty_list = Vec::new();
            return gather_beer_list(rows, empty_list);
        }
    }
}
