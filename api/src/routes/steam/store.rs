use rocket::serde::json::Json;
use crate::response::ApiResponse;
use rocket::{get, Route};
use serde_json::Value;
use scraper::{Html, Selector};

#[get("/apps?<query>&<page>&<count>&<cc>&<language>")]
pub async fn games(
    query: Option<String>,
    page: Option<u32>,
    count: Option<u32>,
    cc: Option<String>,
    language: Option<String>
) -> Json<ApiResponse<Value>> {
    let mut url = String::from("https://store.steampowered.com/search/results/?norender=1");
    if let Some(q) = query {
        url.push_str(&format!("&term={}", urlencoding::encode(&q)));
    }
    url.push_str(&format!("&page={}", page.unwrap_or(1)));
    url.push_str(&format!("&count={}", count.unwrap_or(10)));
    if let Some(country) = cc {
        url.push_str(&format!("&cc={}", urlencoding::encode(&country)));
    }
    if let Some(lang) = language {
        url.push_str(&format!("&l={}", urlencoding::encode(&lang)));
    }
    let resp = match reqwest::get(&url).await {
        Ok(r) => r,
        Err(e) => return Json(ApiResponse::new(
            500,
            false,
            "Error making request".to_string(),
            None,
            None,
            chrono::Utc::now().to_rfc3339(),
            Some(e.to_string()),
        )),
    };
    let html = match resp.text().await {
        Ok(t) => t,
        Err(e) => return Json(ApiResponse::new(
            500,
            false,
            "Error reading HTML response".to_string(),
            None,
            None,
            chrono::Utc::now().to_rfc3339(),
            Some(e.to_string()),
        )),
    };
    let document = Html::parse_document(&html);
    let selector = Selector::parse("a.search_result_row").unwrap();
    let mut results = Vec::new();
    for game in document.select(&selector) {
        let title = game.select(&Selector::parse("span.title").unwrap()).next().map(|e| e.text().collect::<String>());
        let appid = game.value().attr("data-ds-appid").and_then(|v| v.parse::<u32>().ok());
        let url = game.value().attr("href").map(|s| s.to_string());
        let img = game.select(&Selector::parse(".search_capsule img").unwrap()).next().and_then(|e| e.value().attr("src")).map(|s| s.to_string());
        let price_final = game.select(&Selector::parse(".discount_final_price").unwrap()).next().map(|e| e.text().collect::<String>());
        let price_original = game.select(&Selector::parse(".discount_original_price").unwrap()).next().map(|e| e.text().collect::<String>());
        let discount_pct = game.select(&Selector::parse(".discount_pct").unwrap()).next().map(|e| e.text().collect::<String>());
        let released = game.select(&Selector::parse(".search_released").unwrap()).next().map(|e| e.text().collect::<String>());
        let review = game.select(&Selector::parse(".search_review_summary").unwrap()).next().and_then(|e| e.value().attr("data-tooltip-html")).map(|s| s.to_string());
        let mut platforms = Vec::new();
        let platform_selector = Selector::parse(".platform_img").unwrap();
        let platform_div = game.select(&platform_selector);
        for p in platform_div {
            if let Some(class) = p.value().attr("class") {
                if class.contains("win") { platforms.push("windows"); }
                if class.contains("mac") { platforms.push("mac"); }
                if class.contains("linux") { platforms.push("linux"); }
            }
        }
        let tags = game.value().attr("data-ds-tagids").map(|s| Value::String(s.to_string()));
        let descids = game.value().attr("data-ds-descids").map(|s| Value::String(s.to_string()));
        let crtrids = game.value().attr("data-ds-crtrids").map(|s| Value::String(s.to_string()));
        let itemkey = game.value().attr("data-ds-itemkey").map(|s| Value::String(s.to_string()));
        let steamdeck = game.value().attr("data-ds-steam-deck-compat-handled").map(|s| Value::String(s.to_string()));
        let bundle_discount = game.select(&Selector::parse(".discount_block").unwrap()).next().and_then(|e| e.value().attr("data-bundlediscount")).map(|s| s.to_string());
        let mut obj = serde_json::Map::new();
        if let Some(appid) = appid {
            obj.insert("appid".to_string(), Value::Number(appid.into()));
            let img_large = format!("https://shared.fastly.steamstatic.com/store_item_assets/steam/apps/{}/header.jpg", appid);
            obj.insert("img_large".to_string(), Value::String(img_large));
        }
        if let Some(title) = title { obj.insert("title".to_string(), Value::String(title)); }
        if let Some(url) = url { obj.insert("url".to_string(), Value::String(url)); }
        if let Some(img) = img { obj.insert("img".to_string(), Value::String(img)); }
        if let Some(price_final) = price_final { obj.insert("price_final".to_string(), Value::String(price_final)); }
        if let Some(price_original) = price_original { obj.insert("price_original".to_string(), Value::String(price_original)); }
        let mut discount_value = 0;
        if let Some(discount_pct) = discount_pct {
            obj.insert("discount_pct".to_string(), Value::String(discount_pct.clone()));
            obj.insert("discounted".to_string(), Value::Bool(true));
            if let Some(stripped) = discount_pct.strip_prefix('-').and_then(|s| s.strip_suffix('%')) {
                if let Ok(val) = stripped.parse::<u32>() {
                    discount_value = val;
                }
            }
        } else {
            obj.insert("discount_pct".to_string(), Value::String("0%".to_string()));
            obj.insert("discounted".to_string(), Value::Bool(false));
        }
        obj.insert("discount".to_string(), Value::Number(discount_value.into()));
        if let Some(bundle_discount) = bundle_discount { obj.insert("bundle_discount".to_string(), Value::String(bundle_discount)); }
        if let Some(released) = released { obj.insert("released".to_string(), Value::String(released)); }
        if let Some(review) = review { obj.insert("review".to_string(), Value::String(review)); }
        if !platforms.is_empty() { obj.insert("platforms".to_string(), Value::Array(platforms.into_iter().map(|s| Value::String(s.to_string())).collect())); }
        if let Some(tags) = tags { obj.insert("tags".to_string(), tags); }
        if let Some(descids) = descids { obj.insert("descids".to_string(), descids); }
        if let Some(crtrids) = crtrids { obj.insert("crtrids".to_string(), crtrids); }
        if let Some(itemkey) = itemkey { obj.insert("itemkey".to_string(), itemkey); }
        if let Some(steamdeck) = steamdeck { obj.insert("steamdeck".to_string(), steamdeck); }
        results.push(Value::Object(obj));
    }
    let size = Some(results.len() as u64);
    Json(ApiResponse::new(
        200,
        true,
        "OK".to_string(),
        size,
        Some(Value::Array(results)),
        chrono::Utc::now().to_rfc3339(),
        None,
    ))
}

#[get("/app/<appid>?<language>&<cc>")]
pub async fn game(appid: u32, language: Option<String>, cc: Option<String>) -> Json<ApiResponse<Value>> {
    let mut url = format!("https://store.steampowered.com/api/appdetails?appids={}", appid);
    if let Some(lang) = language {
        url.push_str(&format!("&l={}", urlencoding::encode(&lang)));
    }
    if let Some(country) = cc {
        url.push_str(&format!("&cc={}", urlencoding::encode(&country)));
    }
    let resp = match reqwest::get(&url).await {
        Ok(r) => r,
        Err(e) => return Json(ApiResponse::new(
            500,
            false,
            "Error making request".to_string(),
            None,
            None,
            chrono::Utc::now().to_rfc3339(),
            Some(e.to_string()),
        )),
    };
    let json: Value = match resp.json().await {
        Ok(j) => j,
        Err(e) => return Json(ApiResponse::new(
            500,
            false,
            "Error reading JSON response".to_string(),
            None,
            None,
            chrono::Utc::now().to_rfc3339(),
            Some(e.to_string()),
        )),
    };
    let entry = json.get(appid.to_string()).and_then(|v| v.as_object());
    match entry {
        Some(obj) => {
            let success = obj.get("success").and_then(|v| v.as_bool()).unwrap_or(false);
            if !success {
                return Json(ApiResponse::new(
                    404,
                    false,
                    "App not found or invalid appid".to_string(),
                    None,
                    None,
                    chrono::Utc::now().to_rfc3339(),
                    Some("success == false".to_string()),
                ));
            }
            let data = obj.get("data").cloned();
            let size = Some(1);
            Json(ApiResponse::new(
                200,
                true,
                "OK".to_string(),
                size,
                data,
                chrono::Utc::now().to_rfc3339(),
                None,
            ))
        }
        None => Json(ApiResponse::new(
            500,
            false,
            "Malformed response from Steam".to_string(),
            None,
            None,
            chrono::Utc::now().to_rfc3339(),
            Some("appid entry missing".to_string()),
        )),
    }
}

pub fn all_routes() -> Vec<Route> {
    routes![games, game]
}
