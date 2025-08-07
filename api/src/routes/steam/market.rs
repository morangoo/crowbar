fn process_icon_urls(obj: &mut serde_json::Map<String, Value>) {
    if let Some(icon_url_val) = obj.get_mut("icon_url") {
        if let Some(icon_url_str) = icon_url_val.as_str() {
            *icon_url_val = Value::String(make_icon_url(icon_url_str));
        }
    }
    if let Some(icon_url_large_val) = obj.get_mut("icon_url_large") {
        if let Some(icon_url_large_str) = icon_url_large_val.as_str() {
            *icon_url_large_val = Value::String(make_icon_url(icon_url_large_str));
        }
    }
}
fn make_icon_url(s: &str) -> String {
    format!("https://community.fastly.steamstatic.com/economy/image/{}", s)
}
use scraper::{Html, Selector};
use rocket::serde::json::Json;
use crate::response::ApiResponse;
use rocket::{get};
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

    // Build Steam Market search URL
    let mut url = format!("https://steamcommunity.com/market/search/render/?count=10&norender=1&sort={sort}&start={start}");
    if !query.is_empty() {
        url.push_str(&format!("&query={}", urlencoding::encode(query)));
    }
    if !appid.is_empty() {
        url.push_str(&format!("&appid={}", appid));
    }

    // Fetch and parse response
    let resp = match reqwest::get(&url).await {
        Ok(r) => r,
        Err(e) => return Json(ApiResponse::new(500, false, "Error making request".to_string(), None, None, chrono::Utc::now().to_rfc3339(), Some(e.to_string()))),
    };
    let json: Value = match resp.json().await {
        Ok(j) => j,
        Err(e) => return Json(ApiResponse::new(500, false, "Error reading JSON response".to_string(), None, None, chrono::Utc::now().to_rfc3339(), Some(e.to_string()))),
    };

    if json.get("success") != Some(&Value::Bool(true)) {
        return Json(ApiResponse::new(400, false, "Invalid request or unsuccessful response".to_string(), None, None, chrono::Utc::now().to_rfc3339(), Some("success != true".to_string())));
    }

    let pagesize = json.get("searchdata")
        .and_then(|sd| sd.get("pagesize"))
        .and_then(|v| v.as_u64())
        .unwrap_or(0);
    let mut processed_results = Vec::new();
    if let Some(results) = json.get("results").and_then(|v| v.as_array()) {
        for mut item in results.clone() {
            if let Some(obj) = item.as_object_mut() {
                // Move asset_description to item_details
                if let Some(asset) = obj.remove("asset_description") {
                    obj.insert("item_details".to_string(), asset);
                }
                // Process icon URLs and add market_url
                if let Some(item_obj) = obj.get_mut("item_details") {
                    if let Some(item_map) = item_obj.as_object_mut() {
                        process_icon_urls(item_map);
                        let appid = item_map.get("appid").and_then(|v| v.as_u64());
                        let market_hash_name = item_map.get("market_hash_name").and_then(|v| v.as_str());
                        if let (Some(appid), Some(market_hash_name)) = (appid, market_hash_name) {
                            let url = format!(
                                "https://steamcommunity.com/market/listings/{}/{}",
                                appid,
                                urlencoding::encode(market_hash_name)
                            );
                            item_map.insert("market_url".to_string(), Value::String(url));
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
}

#[get("/item?<appid>&<hashname>")]
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
    let encoded_name = urlencoding::encode(marketname);
    let url = format!(
        "https://steamcommunity.com/market/listings/{}/{}",
        appid, encoded_name
    );
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
    let html = match resp.text().await {
        Ok(t) => t,
        Err(e) => {
            return Json(ApiResponse::new(
                500,
                false,
                "Error reading HTML response".to_string(),
                None,
                None,
                chrono::Utc::now().to_rfc3339(),
                Some(e.to_string()),
            ));
        }
    };
    let document = Html::parse_document(&html);
    let selector = Selector::parse("script").unwrap();
    for script in document.select(&selector) {
        let script_text = script.text().collect::<Vec<_>>().join("");
        if let Some(idx) = script_text.find("g_rgAssets = ") {
            let json_start = idx + "g_rgAssets = ".len();
            if let Some(end_idx) = script_text[json_start..].find(";") {
                let json_str = &script_text[json_start..json_start+end_idx];
                let result = match serde_json::from_str::<Value>(json_str) {
                    Ok(val) => {
                        let mut current = &val;
                        while let Some(map) = current.as_object() {
                            if map.len() == 1 {
                                current = map.values().next().unwrap();
                            } else {
                                break;
                            }
                        }
                        let mut obj = if let Some(id) = current.get("id").and_then(|v| v.as_str()) {
                            current.get(id).cloned().unwrap_or_else(|| current.clone())
                        } else {
                            current.clone()
                        };
                        if let Some(obj_map) = obj.as_object_mut() {
                            process_icon_urls(obj_map);
                        }
                        Ok(obj)
                    },
                    Err(e) => Err(e)
                };
                return match result {
                    Ok(obj) => Json(ApiResponse::new(
                        200,
                        true,
                        "OK".to_string(),
                        None,
                        Some(obj),
                        chrono::Utc::now().to_rfc3339(),
                        None,
                    )),
                    Err(e) => Json(ApiResponse::new(
                        500,
                        false,
                        "Error parsing JSON from script".to_string(),
                        None,
                        None,
                        chrono::Utc::now().to_rfc3339(),
                        Some(e.to_string()),
                    )),
                };
            }
        }
    }
    Json(ApiResponse::new(
        404,
        false,
        "Asset information not found".to_string(),
        None,
        None,
        chrono::Utc::now().to_rfc3339(),
        None,
    ))
}

pub fn all_routes() -> Vec<Route> {
    routes![search, item]
}
