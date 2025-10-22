#[macro_use] extern crate rocket;
mod response;
pub mod maps;
mod catchers;

mod services;

pub mod utils;

mod routes {
    pub mod steam {
        pub mod market;
        pub mod store;
    }
}

use routes::steam::market::all_routes as market_routes;
use routes::steam::store::all_routes as store_routes;
use rocket::Config;
use rocket::http::Status;
use rocket_governor::RocketGovernor;
use crate::utils::rate_limit::RateLimitGuard;

#[get("/")]
fn index(_limitguard: RocketGovernor<RateLimitGuard>) -> Status {
    Status::Ok
}

#[launch]
fn rocket() -> _ {
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "8000".into())
        .parse()
        .expect("PORT must be a number");

    let config = Config {
        address: "0.0.0.0".parse().unwrap(),
        port,
        ..Config::default()
    };

    rocket::build()
        .configure(config)
        .mount("/", routes![index])
        .mount("/api/steam/market", market_routes())
        .mount("/api/steam/", store_routes())
        .register("/", catchers::all_catchers())
}