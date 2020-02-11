mod example;
////////////
mod beer;

pub use example::{
    create_person,
    delete_person_by_id,
    get_name_id_person,
    get_people,
    update_person_by_id,
};

////////////

pub use beer::{
    fetch_beer_list,
};
