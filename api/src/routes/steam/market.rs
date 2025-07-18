use rocket::get;

#[get("/market/ping")]
pub fn ping() -> &'static str {
    "pong from market"
}
