use rocket::serde::json::Json;
use crate::response::ApiResponse;
use rocket::{get};
use rocket::Route;
use serde_json::Value;
use crate::services::steam::market as market_services;
use crate::utils::cache::build_cache_key;
use crate::utils::redis::get_redis_conn_async;
use crate::utils::rate_limit::RateLimitGuard;
use rocket_governor::RocketGovernor;
use redis::AsyncCommands;
use serde::Serialize;

#[derive(Serialize)]
struct SearchParams {
    appid: Option<String>,
    query: Option<String>,
    sort: Option<String>,
    page: Option<u32>,
}

#[get("/search?<appid>&<query>&<sort>&<page>")]
pub async fn search(
    _limitguard: RocketGovernor<'_, RateLimitGuard>,
    appid: Option<String>,
    query: Option<String>,
    sort: Option<String>,
    page: Option<u32>,
) -> Json<ApiResponse<Value>> {
    let params = SearchParams {
        appid: appid.clone(),
        query: query.clone(),
        sort: sort.clone(),
        page,
    };
    let cache_key = build_cache_key("market_search_cache", &params);

    if let Ok(mut conn) = get_redis_conn_async().await {
        if let Ok(Some(cached_json)) = conn.get::<_, Option<String>>(&cache_key).await {
            if let Ok((size, results)) = serde_json::from_str::<(u64, Vec<Value>)>(&cached_json) {
                return Json(ApiResponse::new(
                    200,
                    true,
                    "OK (cache)".to_string(),
                    Some(size),
                    Some(Value::Array(results)),
                    chrono::Utc::now().to_rfc3339(),
                    None,
                ));
            }
        }
    }

    match market_services::search::fetch_search(appid, query, sort, page).await {
        Ok((pagesize, processed_results)) => {
            if let Ok(mut conn) = get_redis_conn_async().await {
                let _ = conn.set_ex::<_, _, ()>(
                    &cache_key,
                    serde_json::to_string(&(pagesize, &processed_results)).unwrap_or_default(),
                    300,
                ).await;
            }

            Json(ApiResponse::new(
                200,
                true,
                "OK".to_string(),
                Some(pagesize),
                Some(Value::Array(processed_results)),
                chrono::Utc::now().to_rfc3339(),
                None,
            ))
        }
        Err(e) => Json(ApiResponse::new(
            500,
            false,
            "Error fetching search results".to_string(),
            None,
            None,
            chrono::Utc::now().to_rfc3339(),
            Some(e),
        ))
    }
}

#[derive(Serialize)]
struct ItemParams<'a> {
    appid: &'a str,
    marketname: &'a str,
}

#[get("/item/<appid>?<hashname>")]
pub async fn item(_limitguard: RocketGovernor<'_, RateLimitGuard>, appid: Option<&str>, hashname: Option<&str>) -> Json<ApiResponse<Value>> {
    let appid = appid.unwrap_or("");
    let marketname = hashname.unwrap_or("");

    if appid.is_empty() || marketname.is_empty() {
        return Json(ApiResponse::new(
            400,
            false,
            "Missing appid or marketname".to_string(),
            None,
            None,
            chrono::Utc::now().to_rfc3339(),
            Some("appid or marketname missing".to_string()),
        ));
    }

    let params = ItemParams { appid, marketname };
    let cache_key = build_cache_key("market_item_cache", &params);

    if let Ok(mut conn) = get_redis_conn_async().await {
        if let Ok(Some(cached_json)) = conn.get::<_, Option<String>>(&cache_key).await {
            if let Ok(item) = serde_json::from_str::<Value>(&cached_json) {
                return Json(ApiResponse::new(
                    200,
                    true,
                    "OK (cache)".to_string(),
                    None,
                    Some(item),
                    chrono::Utc::now().to_rfc3339(),
                    None,
                ));
            }
        }
    }

    match market_services::item::fetch_item(appid, marketname).await {
        Ok(item) => {
            if let Ok(mut conn) = get_redis_conn_async().await {
                let _ = conn.set_ex::<_, _, ()>(
                    &cache_key,
                    serde_json::to_string(&item).unwrap_or_default(),
                    300,
                ).await;
            }

            Json(ApiResponse::new(
                200,
                true,
                "OK".to_string(),
                None,
                Some(item),
                chrono::Utc::now().to_rfc3339(),
                None,
            ))
        },
        Err(e) => Json(ApiResponse::new(
            404,
            false,
            "Error fetching item".to_string(),
            None,
            None,
            chrono::Utc::now().to_rfc3339(),
            Some(e),
        ))
    }
}

pub fn all_routes() -> Vec<Route> {
    routes![search, item]
}
