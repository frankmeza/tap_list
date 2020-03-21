use crate::models::Beer;
use tokio_postgres::{row::Row, Error};

pub fn gather_beer_list(rows: Vec<Row>, mut beer_list: Vec<Beer>) -> Result<Vec<Beer>, Error> {
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
            keg_id: row.get(12),
            keg_size: row.get(13),
            keg_amount_left: row.get(14),
            updated_ts: row.get(15),
            created_ts: row.get(16),
        };

        beer_list.push(beer);
    }

    Ok(beer_list)
}

// pub fn pour_a_beer() -> Beer {
//     Beer {
//         id: 0,
//         name: String::new(),
//         beer_type: String::new(),
//         abv: String::new(),
//         ibu: String::new(),
//         serving_size: String::new(),
//         cost: String::new(),
//         brewery_name: String::new(),
//         brewery_city: String::new(),
//         brewery_state: String::new(),
//         keg_id: String::new(),
//         keg_size: 0,
//         keg_amount_left: 0,
//         updated_ts: 0,
//         created_ts: 0,
//     }
// }
