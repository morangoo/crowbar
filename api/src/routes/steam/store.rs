use rocket::serde::{json::Json, Deserialize};
use crate::response::ApiResponse;
use rocket::{get, Route, post};
use serde_json::Value;
use crate::services::steam as steam_services;

#[derive(Deserialize)]
pub struct AppsRequest {
    pub query: Option<String>,
    pub page: Option<u32>,
    pub count: Option<u32>,
    pub cc: Option<String>,
    pub language: Option<String>,
    pub tags: Option<Vec<u32>>,
}

#[post("/apps", data = "<body>")]
pub async fn apps(body: Json<AppsRequest>) -> Json<ApiResponse<Value>> {
    let query = body.query.clone();
    let page = body.page;
    let count = body.count;
    let cc = body.cc.clone();
    let language = body.language.clone();
    let tags = body.tags.clone();

    match steam_services::fetch_apps(query, page, count, cc, language, tags).await {
        Ok(val) => {
            let size = val.as_array().map(|a| a.len() as u64).unwrap_or(0);
            Json(ApiResponse::new(200, true, "OK".to_string(), Some(size), Some(val), chrono::Utc::now().to_rfc3339(), None))
        }
        Err(e) => Json(ApiResponse::new(500, false, "Error fetching apps".to_string(), None, None, chrono::Utc::now().to_rfc3339(), Some(e))),
    }
}

#[get("/app/<appid>?<language>&<cc>")]
pub async fn app(appid: u32, language: Option<String>, cc: Option<String>) -> Json<ApiResponse<Value>> {
    match steam_services::fetch_app(appid, language, cc).await {
        Ok(val) => Json(ApiResponse::new(200, true, "OK".to_string(), Some(1), Some(val), chrono::Utc::now().to_rfc3339(), None)),
        Err(e) => Json(ApiResponse::new(500, false, "Error fetching app".to_string(), None, None, chrono::Utc::now().to_rfc3339(), Some(e))),
    }
}

pub fn all_routes() -> Vec<Route> {
    routes![apps, app]
}