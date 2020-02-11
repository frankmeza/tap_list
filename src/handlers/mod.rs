mod example;
////////////
mod beer;

pub use example::{
    fetch_people_list,
    fetch_person_by_id,
    create_person,
    update_person_by_id,
    delete_person_by_id,
};

////////////

pub use beer::{
    handle_fetch_beer_list,
};
