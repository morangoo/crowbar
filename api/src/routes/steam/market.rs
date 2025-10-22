use rocket::serde::json::Json;
use crate::response::ApiResponse;
use rocket::{get};
use rocket::Route;
use serde_json::Value;
use crate::services::steam::market as market_services;

#[get("/search?<appid>&<query>&<sort>&<page>")]
pub async fn search(
    appid: Option<String>,
    query: Option<String>,
    sort: Option<String>,
    page: Option<u32>,
) -> Json<ApiResponse<Value>> {
    match market_services::search::fetch_search(appid, query, sort, page).await {
        Ok((pagesize, processed_results)) => {
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

#[get("/item/<appid>?<hashname>")]
pub async fn item(appid: Option<&str>, hashname: Option<&str>) -> Json<ApiResponse<Value>> {
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

    match market_services::item::fetch_item(appid, marketname).await {
        Ok(item) => Json(ApiResponse::new(
            200,
            true,
            "OK".to_string(),
            None,
            Some(item),
            chrono::Utc::now().to_rfc3339(),
            None,
        )),
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
