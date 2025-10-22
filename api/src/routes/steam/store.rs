use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use crate::response::ApiResponse;
use rocket::{get, Route, post};
use serde_json::Value;
use crate::services::steam as steam_services;
use crate::utils::cache::build_cache_key;
use crate::utils::redis::get_redis_conn_async;
use crate::utils::rate_limit::RateLimitGuard;
use rocket_governor::RocketGovernor;
use redis::AsyncCommands;

#[derive(Deserialize, Serialize)]
pub struct AppsRequest {
    pub query: Option<String>,
    pub page: Option<u32>,
    pub count: Option<u32>,
    pub cc: Option<String>,
    pub language: Option<String>,
    pub tags: Option<Vec<u32>>,
}

#[post("/apps", data = "<body>")]
pub async fn apps(_limitguard: RocketGovernor<'_, RateLimitGuard>, body: Json<AppsRequest>) -> Json<ApiResponse<Value>> {

    let cache_key = build_cache_key("apps_cache", &body.0);

    if let Ok(mut conn) = get_redis_conn_async().await {
        if let Ok(Some(cached_json)) = conn.get::<_, Option<String>>(&cache_key).await {
            if let Ok(val) = serde_json::from_str::<Value>(&cached_json) {
                let size = val.as_array().map(|a| a.len() as u64).unwrap_or(0);
                return Json(ApiResponse::new(200, true, "OK (cache)".to_string(), Some(size), Some(val), chrono::Utc::now().to_rfc3339(), None));
            }
        }
    }

    let query = body.query.clone();
    let page = body.page;
    let count = body.count;
    let cc = body.cc.clone();
    let language = body.language.clone();
    let tags = body.tags.clone();

    match steam_services::fetch_apps(query, page, count, cc, language, tags).await {
        Ok(val) => {
            let size = val.as_array().map(|a| a.len() as u64).unwrap_or(0);

            if let Ok(mut conn) = get_redis_conn_async().await {
                let _ = conn.set_ex::<_, _, ()>(&cache_key, serde_json::to_string(&val).unwrap_or_default(), 300).await;
            }

            Json(ApiResponse::new(200, true, "OK".to_string(), Some(size), Some(val), chrono::Utc::now().to_rfc3339(), None))
        }
        Err(e) => Json(ApiResponse::new(500, false, "Error fetching apps".to_string(), None, None, chrono::Utc::now().to_rfc3339(), Some(e))),
    }
}

#[get("/app/<appid>?<language>&<cc>")]
pub async fn app(_limitguard: RocketGovernor<'_, RateLimitGuard>, appid: u32, language: Option<String>, cc: Option<String>) -> Json<ApiResponse<Value>> {

    let key_input = (appid, language.clone(), cc.clone());
    let cache_key = build_cache_key("app_cache", &key_input);

    if let Ok(mut conn) = get_redis_conn_async().await {
        if let Ok(Some(cached_json)) = conn.get::<_, Option<String>>(&cache_key).await {
            if let Ok(val) = serde_json::from_str::<Value>(&cached_json) {
                return Json(ApiResponse::new(200, true, "OK (cache)".to_string(), Some(1), Some(val), chrono::Utc::now().to_rfc3339(), None));
            }
        }
    }

    match steam_services::fetch_app(appid, language, cc).await {
        Ok(val) => {
            if let Ok(mut conn) = get_redis_conn_async().await {
                let _ = conn.set_ex::<_, _, ()>(&cache_key, serde_json::to_string(&val).unwrap_or_default(), 300).await;
            }
            Json(ApiResponse::new(200, true, "OK".to_string(), Some(1), Some(val), chrono::Utc::now().to_rfc3339(), None))
        }
        Err(e) => Json(ApiResponse::new(500, false, "Error fetching app".to_string(), None, None, chrono::Utc::now().to_rfc3339(), Some(e))),
    }
}

pub fn all_routes() -> Vec<Route> {
    routes![apps, app]
}