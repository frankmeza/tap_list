use crate::{models::Beer, queries};
use postgres::Connection;

mod utils;

pub fn fetch_beer_list(conn: Connection) -> Vec<Beer> {
    let mut beers = Vec::new();
    let q = queries::get_beers();
    let rows = &conn.query(&q, &[]);

    match rows {
        Err(err) => {
            println!("rows in fetch_beer_list very virus: {:?}", err);
        }
        Ok(rows) => {
            for row in rows {
                let beer = Beer {
                    id: row.get(0),
                    name: row.get(1),
                    beer_type: row.get(2),
                    abv: row.get(3),
                    ibu: row.get(4),
                    serving_size: row.get(5),
                    cost: row.get(6),
                    brewery_name: row.get(7),
                    brewery_city: row.get(8),
                    brewery_state: row.get(9),
                    brewery_img_url: row.get(10),
                    keg_id: row.get(11),
                    keg_size: row.get(12),
                    keg_amount_left: row.get(13),
                    updated_ts: row.get(14),
                    created_ts: row.get(15),
                };

                beers.push(beer);
            }
        }
    }

    beers
}

pub fn fetch_beer_by_id(conn: Connection, id: &str) -> Beer {
    let mut beer = utils::pour_a_beer();
    let q = queries::get_beer_by_id(id);
    let rows = &conn.query(&q, &[]);

    match rows {
        Err(err) => {
            println!("rows in fetch_person_by_id very virus: {:?}", err);
        }
        Ok(rows) => {
            for row in rows {
                let b = Beer {
                    id: row.get(0),
                    name: row.get(1),
                    beer_type: row.get(2),
                    abv: row.get(3),
                    ibu: row.get(4),
                    serving_size: row.get(5),
                    cost: row.get(6),
                    brewery_name: row.get(7),
                    brewery_city: row.get(8),
                    brewery_state: row.get(9),
                    brewery_img_url: row.get(10),
                    keg_id: row.get(11),
                    keg_size: row.get(12),
                    keg_amount_left: row.get(13),
                    updated_ts: row.get(14),
                    created_ts: row.get(15),
                };

                beer = b;
            }
        }
    }

    beer
}

pub fn create_person(conn: Connection, id: &str, name: &str, timestamp: u64) {
    let q = queries::create_person(id, name, timestamp);
    let rows = &conn.query(&q, &[]);

    match rows {
        Ok(_rows) => (),
        Err(err) => {
            println!("rows in create_person very virus: {:?}", err);
        }
    }
}

pub fn update_person_by_id(conn: Connection, id: &str, name: &str) {
    let q = queries::update_person_by_id(id, name);
    let rows = &conn.query(&q, &[]);

    match rows {
        Ok(_rows) => (),
        Err(err) => {
            println!("rows in update_person_by_id very virus: {:?}", err);
        }
    }
}

pub fn delete_person_by_id(conn: Connection, id: &str) {
    let q = queries::delete_person_by_id(id);
    let rows = &conn.query(&q, &[]);

    match rows {
        Ok(_rows) => (),
        Err(err) => {
            println!("rows in delete_person_by_id very virus: {:?}", err);
        }
    }
}
