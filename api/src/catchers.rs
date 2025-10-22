//use rocket::http::Status;
use rocket::serde::json::Json;
use crate::response::ApiResponse;

#[catch(429)]
pub fn too_many_requests() -> Json<ApiResponse<()>> {
    Json(ApiResponse::new(
        429,
        false,
        "Too Many Requests".to_string(),
        None,
        None,
        chrono::Utc::now().to_rfc3339(),
        Some("Rate limit exceeded. Please try again later.".to_string()),
    ))
}

pub fn all_catchers() -> Vec<rocket::Catcher> {
    catchers![too_many_requests]
}
