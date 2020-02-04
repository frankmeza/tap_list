use crate::models::Beer;

pub fn pour_a_beer() -> Beer {
    Beer {
        id: 0,
        name: String::new(),
        beer_type: String::new(),
        abv: String::new(),
        ibu: String::new(),
        serving_size: String::new(),
        cost: String::new(),
        brewery_name: String::new(),
        brewery_city: String::new(),
        brewery_state: String::new(),
        brewery_img_url: String::new(),
        keg_id: String::new(),
        keg_size: 0,
        keg_amount_left: 0,
        updated_ts: 0,
        created_ts: 0,
    }
}
