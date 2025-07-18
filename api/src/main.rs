#[macro_use] extern crate rocket;

mod routes {
    pub mod steam {
        pub mod market;
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello World"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/api/steam/market", routes![routes::steam::market::ping])
}


