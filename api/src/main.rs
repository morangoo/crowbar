#[macro_use] extern crate rocket;
mod response;

mod routes {
    pub mod steam {
        pub mod market;
    }
}

use routes::steam::market::all_routes;

#[get("/")]
fn index() -> &'static str {
    "Hello World"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/api/steam/market", all_routes())
}


