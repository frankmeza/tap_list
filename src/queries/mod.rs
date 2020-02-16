// EXAMPLE
mod example;

pub use example::{
    create_person,
    delete_person_by_id,
    get_name_id_person,
    get_people,
    update_person_by_id,
};

//////////

mod beer;
mod beer_utils;

pub use beer::{
    get_beer_list,
};
