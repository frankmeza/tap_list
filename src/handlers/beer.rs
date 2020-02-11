use postgres::Connection;

use crate::{models::*, queries};

pub fn handle_fetch_beer_list(conn: Connection) -> Vec<Beer> {
    let mut beer_list = Vec::new();
    let q = queries::fetch_beer_list();
    let rows = &conn.query(&q, &[]);

    match rows {
        Err(err) => {
            println!("fetch_beer_list very virus: {:?}", err);
        }
        Ok(rows) => {
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
        }
    }

    beer_list
}
