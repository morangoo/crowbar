use rocket::serde::json::Json;
use crate::response::ApiResponse;
use rocket::{get, Route};
use serde_json::Value;

#[get("/games")]
pub async fn games() -> Json<ApiResponse<Value>> {
    Json(ApiResponse::new(
        200,
        true,
        "OK".to_string(),
        None,
        None,
        chrono::Utc::now().to_rfc3339(),
        None,
    ))
}

pub fn all_routes() -> Vec<Route> {
    routes![games]
}
