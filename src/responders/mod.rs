mod example;
////////////
mod beer;

pub use example::{
    health_check,
    get_people_list,
    create_person,
    get_person_by_id,
    update_person_by_id,
    delete_person_by_id,
};

////////////////////////
pub use beer::{
    get_beer_list,
};
