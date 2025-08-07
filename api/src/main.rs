#[macro_use] extern crate rocket;
mod response;

mod routes {
    pub mod steam {
        pub mod market;
        pub mod store;
    }
}

use routes::steam::market::all_routes as market_routes;
use routes::steam::store::all_routes as store_routes;

#[get("/")]
fn index() -> &'static str {
    "Hello World"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/api/steam/market", market_routes())
        .mount("/api/steam/store", store_routes())
}


