mod app;
mod remote;

// EXAMPLE
pub use app::Person;
//////////
pub use app::{Beer, BeerFilters};

// EXAMPLE
pub use remote::{Id, PersonReq};
//////////
pub use remote::BeerRequest;
