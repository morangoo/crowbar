use rocket::serde::json::Json;
use crate::response::ApiResponse;
use rocket::get;
use rocket::Route;
use serde_json::Value;

#[get("/search?<appid>&<query>&<sort>&<page>")]
pub async fn search(
    appid: Option<String>,
    query: Option<String>,
    sort: Option<String>,
    page: Option<u32>,
    ) -> Json<ApiResponse<Value>> {
    let appid = appid.as_deref().unwrap_or("");
    let query = query.as_deref().unwrap_or("");
    let sort = sort.as_deref().unwrap_or("default_desc");
    let page = page.unwrap_or(1);
    let start = (page - 1) * 10;

    let mut url = format!("https://steamcommunity.com/market/search/render/?count=10&norender=1&sort={sort}&start={start}");
    if !query.is_empty() {
        url.push_str(&format!("&query={}", urlencoding::encode(query)));
    }
    if !appid.is_empty() {
        url.push_str(&format!("&appid={}", appid));
    }

    let resp = match reqwest::get(&url).await {
        Ok(r) => r,
        Err(e) => {
            return Json(ApiResponse::new(
                500,
                false,
                "Error making request".to_string(),
                None,
                None,
                chrono::Utc::now().to_rfc3339(),
                Some(e.to_string()),
            ));
        }
    };
    let json: Value = match resp.json().await {
        Ok(j) => j,
        Err(e) => {
            return Json(ApiResponse::new(
                500,
                false,
                "Error reading JSON response".to_string(),
                None,
                None,
                chrono::Utc::now().to_rfc3339(),
                Some(e.to_string()),
            ));
        }
    };

    if json.get("success") == Some(&Value::Bool(true)) {
        let pagesize = json.get("searchdata")
            .and_then(|sd| sd.get("pagesize"))
            .and_then(|v| v.as_u64())
            .unwrap_or(0);
        let mut processed_results = Vec::new();
        if let Some(arr) = json.get("results").and_then(|v| v.as_array()) {
            for mut item in arr.clone() {
                if let Some(obj) = item.as_object_mut() {
                    if let Some(asset) = obj.remove("asset_description") {
                        obj.insert("item_details".to_string(), asset);
                    }
                    if let Some(item_obj) = obj.get_mut("item_details") {
                        if let Some(item_map) = item_obj.as_object_mut() {
                            if let Some(icon_url_val) = item_map.get_mut("icon_url") {
                                if let Some(icon_url_str) = icon_url_val.as_str() {
                                    let new_url = format!("https://community.fastly.steamstatic.com/economy/image/{}", icon_url_str);
                                    *icon_url_val = Value::String(new_url);
                                }
                            }
                        }
                    }
                }
                processed_results.push(item);
            }
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
    } else {
        Json(ApiResponse::new(
            400,
            false,
            "Invalid request or unsuccessful response".to_string(),
            None,
            None,
            chrono::Utc::now().to_rfc3339(),
            Some("success != true".to_string()),
        ))
    }
}

pub fn all_routes() -> Vec<Route> {
    routes![search]
}
