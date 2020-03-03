use crate::{get_async_connection, models::Beer, queries::get_beer_list};
use tokio_postgres::Error;

pub async fn fetch_beer_list() -> Result<Vec<Beer>, Error> {
    let statement = get_beer_list();
    let rows_future = get_async_connection(statement);

    let beer_list = match rows_future.await {
        Err(tokio_error) => return Err(tokio_error),
        Ok(rows) => {
            let mut beer_list = Vec::new();

            for row in rows {
                let beer = Beer {
                    id: row.get(0),
                    sort_order: row.get(1),
                    name: row.get(2),
                    beer_type: row.get(3),
                    abv: row.get(4),
                    ibu: row.get(5),
                    serving_size: row.get(6),
                    cost: row.get(7),
                    brewery_name: row.get(8),
                    brewery_city: row.get(9),
                    brewery_state: row.get(10),
                    brewery_img_url: row.get(11),
                    keg_id: row.get(12),
                    keg_size: row.get(13),
                    keg_amount_left: row.get(14),
                    updated_ts: row.get(15),
                    created_ts: row.get(16),
                };

                beer_list.push(beer);
            }
            beer_list
        }
    };

    Ok(beer_list)
}
