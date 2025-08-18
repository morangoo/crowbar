use rocket::serde::json::Json;
use crate::response::ApiResponse;
use rocket::{get, Route};
use serde_json::Value;
use scraper::{Html, Selector};
use regex;
// Helper: Extract value from input element by id
fn extract_input_value(document: &Html, id: &str) -> Option<u64> {
    let selector = Selector::parse(&format!("input#{}", id)).ok()?;
    document.select(&selector).next()?.value().attr("value")?.parse::<u64>().ok()
}

// Helper: Transform compatibility items array
fn transform_compat_items(items: &Value) -> Value {
    match items {
        Value::Array(arr) => Value::Array(
            arr.iter().cloned().map(|mut item| {
                if let Value::Object(ref mut obj) = item {
                    if let Some(display_type) = obj.remove("display_type") {
                        obj.insert("compatibility".to_string(), display_type);
                    }
                    if let Some(loc_token) = obj.remove("loc_token") {
                        let token_str = loc_token.as_str().map(|s| s.trim_start_matches('#').to_string()).unwrap_or_default();
                        obj.insert("token".to_string(), Value::String(token_str));
                    }
                }
                item
            }).collect()
        ),
        _ => items.clone(),
    }
}

#[get("/apps?<query>&<page>&<count>&<cc>&<language>")]
pub async fn apps(
    query: Option<String>,
    page: Option<u32>,
    count: Option<u32>,
    cc: Option<String>,
    language: Option<String>
) -> Json<ApiResponse<Value>> {
    // Build Steam search URL
    let mut url = String::from("https://store.steampowered.com/search/results/?norender=1");
    if let Some(q) = query {
        url.push_str(&format!("&term={}", urlencoding::encode(&q)));
    }
    url.push_str(&format!("&page={}", page.unwrap_or(1)));
    url.push_str(&format!("&count={}", count.unwrap_or(10)));
    if let Some(country) = cc {
        url.push_str(&format!("&cc={}", urlencoding::encode(&country)));
    }
    if let Some(ref lang) = language {
        url.push_str(&format!("&l={}", urlencoding::encode(lang)));
    }
    // Fetch search results HTML
    let resp = match reqwest::get(&url).await {
        Ok(r) => r,
        Err(e) => return Json(ApiResponse::new(500, false, "Error making request".to_string(), None, None, chrono::Utc::now().to_rfc3339(), Some(e.to_string()))),
    };
    let html = match resp.text().await {
        Ok(t) => t,
        Err(e) => return Json(ApiResponse::new(500, false, "Error reading HTML response".to_string(), None, None, chrono::Utc::now().to_rfc3339(), Some(e.to_string()))),
    };
    // Parse HTML and extract game results
    let document = Html::parse_document(&html);
    let selector = Selector::parse("a.search_result_row").unwrap();
    let mut results = Vec::new();
    for game in document.select(&selector) {
        let mut obj = serde_json::Map::new();
        // Basic info
        if let Some(appid) = game.value().attr("data-ds-appid").and_then(|v| v.parse::<u32>().ok()) {
            obj.insert("appid".to_string(), Value::Number(appid.into()));
            obj.insert("img_large".to_string(), Value::String(format!("https://shared.fastly.steamstatic.com/store_item_assets/steam/apps/{}/header.jpg", appid)));
        }
        obj.insert("title".to_string(), game.select(&Selector::parse("span.title").unwrap()).next().map(|e| e.text().collect::<String>()).unwrap_or_default().into());
        obj.insert("url".to_string(), game.value().attr("href").map(|s| Value::String(s.to_string())).unwrap_or(Value::Null));
        obj.insert("img".to_string(), game.select(&Selector::parse(".search_capsule img").unwrap()).next().and_then(|e| e.value().attr("src")).map(|s| Value::String(s.to_string())).unwrap_or(Value::Null));
        // Price and discount
        let price_final = game.select(&Selector::parse(".discount_final_price").unwrap()).next().map(|e| e.text().collect::<String>());
        let price_original = game.select(&Selector::parse(".discount_original_price").unwrap()).next().map(|e| e.text().collect::<String>());
        obj.insert("price_final".to_string(), price_final.clone().map(Value::String).unwrap_or(Value::Null));
        obj.insert("price_original".to_string(), price_original.clone().map(Value::String).unwrap_or(Value::Null));
        let discount_pct = game.select(&Selector::parse(".discount_pct").unwrap()).next().map(|e| e.text().collect::<String>());
        let discount_value = discount_pct.as_ref().and_then(|d| d.strip_prefix('-').and_then(|s| s.strip_suffix('%')).and_then(|s| s.parse::<u32>().ok())).unwrap_or(0);
        obj.insert("discount_pct".to_string(), Value::String(discount_pct.clone().unwrap_or("0%".to_string())));
        obj.insert("discounted".to_string(), Value::Bool(discount_pct.is_some()));
        obj.insert("discount".to_string(), Value::Number(discount_value.into()));
        obj.insert("bundle_discount".to_string(), game.select(&Selector::parse(".discount_block").unwrap()).next().and_then(|e| e.value().attr("data-bundlediscount")).map(|s| Value::String(s.to_string())).unwrap_or(Value::Null));
        // Other info
        obj.insert("released".to_string(), game.select(&Selector::parse(".search_released").unwrap()).next().map(|e| Value::String(e.text().collect())).unwrap_or(Value::Null));
        obj.insert("review".to_string(), game.select(&Selector::parse(".search_review_summary").unwrap()).next().and_then(|e| e.value().attr("data-tooltip-html")).map(|s| Value::String(s.to_string())).unwrap_or(Value::Null));
        // Platforms
        let mut platforms = Vec::new();
        for p in game.select(&Selector::parse(".platform_img").unwrap()) {
            if let Some(class) = p.value().attr("class") {
                if class.contains("win") { platforms.push(Value::String("windows".to_string())); }
                if class.contains("mac") { platforms.push(Value::String("mac".to_string())); }
                if class.contains("linux") { platforms.push(Value::String("linux".to_string())); }
            }
        }
        if !platforms.is_empty() { obj.insert("platforms".to_string(), Value::Array(platforms)); }
        // Extra attributes
        for &(key, attr) in &[ ("tags", "data-ds-tagids"), ("descids", "data-ds-descids"), ("crtrids", "data-ds-crtrids"), ("itemkey", "data-ds-itemkey"), ("steamdeck", "data-ds-steam-deck-compat-handled") ] {
            if let Some(val) = game.value().attr(attr) {
                obj.insert(key.to_string(), Value::String(val.to_string()));
            }
        }
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
pub async fn app(appid: u32, language: Option<String>, cc: Option<String>) -> Json<ApiResponse<Value>> {
    let mut url = format!("https://store.steampowered.com/api/appdetails?appids={}", appid);
    if let Some(ref lang) = language {
        url.push_str(&format!("&l={}", urlencoding::encode(lang)));
    }
    if let Some(country) = cc {
        url.push_str(&format!("&cc={}", urlencoding::encode(&country)));
    }
    // Fetch API data
    let resp = match reqwest::get(&url).await {
        Ok(r) => r,
        Err(e) => return Json(ApiResponse::new(500, false, "Error making request".to_string(), None, None, chrono::Utc::now().to_rfc3339(), Some(e.to_string()))),
    };
    let json: Value = match resp.json().await {
        Ok(j) => j,
        Err(e) => return Json(ApiResponse::new(500, false, "Error reading JSON response".to_string(), None, None, chrono::Utc::now().to_rfc3339(), Some(e.to_string()))),
    };
    let entry = json.get(appid.to_string()).and_then(|v| v.as_object());
    match entry {
        Some(obj) => {
            // Check if app exists
            if !obj.get("success").and_then(|v| v.as_bool()).unwrap_or(false) {
                return Json(ApiResponse::new(404, false, "App not found or invalid appid".to_string(), None, None, chrono::Utc::now().to_rfc3339(), Some("success == false".to_string())));
            }
            let mut data = obj.get("data").cloned();

            // Fetch app HTML for extra info, bypassing agecheck with cookies
            let app_url = format!("https://store.steampowered.com/app/{}", appid);
            let client = reqwest::Client::new();
            let html_text = match client
                .get(&app_url)
                .header(
                    reqwest::header::COOKIE,
                    "wants_mature_content=1; lastagecheckage=1-January-2000; birthtime=946684801"
                )
                .send()
                .await {
                Ok(html_resp) => match html_resp.text().await {
                    Ok(t) => t,
                    Err(_) => String::new(),
                },
                Err(_) => String::new(),
            };

            // Extract all HTML info before any await (for Send safety)
            let (positive_reviews, total_reviews, resolved_category, category_key) = {
                let document = Html::parse_document(&html_text);
                // Extract review numbers
                let positive_reviews = extract_input_value(&document, "review_summary_num_positive_reviews");
                let total_reviews = extract_input_value(&document, "review_summary_num_reviews");
                // Extract Steam Deck compatibility info
                let mut resolved_category: Option<serde_json::Value> = None;
                let mut category_key: Option<String> = None;
                if let Some(div) = document.select(&Selector::parse("div#application_config").unwrap()).next() {
                    if let Some(deckcompat) = div.value().attr("data-deckcompatibility") {
                        if let Ok(deck_json) = serde_json::from_str::<serde_json::Value>(deckcompat) {
                            if let Some(cat) = deck_json.get("resolved_category") {
                                resolved_category = Some(cat.clone());
                                if let Some(cat_num) = cat.as_u64() {
                                    let compat_map = crate::maps::steamdeck_compat_map::steamdeck_compatibility_map();
                                    if let Some(key) = compat_map.get(&(cat_num as u8)) {
                                        category_key = Some(key.to_string());
                                    }
                                }
                            }
                            // Transform compatibility items
                            let resolved_items = deck_json.get("resolved_items");
                            let steamos_resolved_items = deck_json.get("steamos_resolved_items");
                            if let Some(Value::Object(ref mut map)) = data {
                                if let Some(items) = resolved_items {
                                    map.insert("steamdeck_compatibility_items".to_string(), transform_compat_items(items));
                                }
                                if let Some(items) = steamos_resolved_items {
                                    map.insert("steamos_resolved_items".to_string(), transform_compat_items(items));
                                }
                            }
                        }
                    }
                }
                (positive_reviews, total_reviews, resolved_category, category_key)
            };

            // Fetch current_players from Steam Community with agecheck cookies
            let community_url = format!("https://steamcommunity.com/app/{}", appid);
            let client = reqwest::Client::new();
            let current_players = match client
                .get(&community_url)
                .header(
                    reqwest::header::COOKIE,
                    "wants_mature_content=1; lastagecheckage=1-January-2000; birthtime=946684801"
                )
                .send()
                .await {
                Ok(resp) => match resp.text().await {
                    Ok(html) => {
                        // Use regex to extract number from <span class="apphub_NumInApp">
                        let re = regex::Regex::new(r#"<span class=\"apphub_NumInApp\">([\d][\d.,]{3,})"#).unwrap();
                        if let Some(caps) = re.captures(&html) {
                            if let Some(num_str) = caps.get(1) {
                                let clean = num_str.as_str().replace([',','.'], "");
                                if let Ok(num) = clean.parse::<u64>() {
                                    Some(Value::Number(num.into()))
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                    Err(_) => None,
                },
                Err(_) => None,
            };

            // Build response JSON
            if let Some(Value::Object(ref mut map)) = data {
                map.insert("cover_image".to_string(), Value::String(format!("https://cdn.cloudflare.steamstatic.com/steam/apps/{}/library_600x900.jpg", appid)));
                if let (Some(pos), Some(total)) = (positive_reviews, total_reviews) {
                    map.insert("positive_reviews".to_string(), Value::Number(pos.into()));
                    map.insert("total_reviews".to_string(), Value::Number(total.into()));
                    let percentage = if total > 0 {
                        let pct = (pos as f64) * 100.0 / (total as f64);
                        format!("{:.2}", pct)
                    } else {
                        "0.00".to_string()
                    };
                    map.insert("positive_reviews_percentage".to_string(), Value::String(percentage));
                }
                // Insert current_players (null if not found)
                map.insert("current_players".to_string(), current_players.unwrap_or(Value::Null));
                if let Some(cat) = resolved_category {
                    map.insert("steamdeck_compatibility".to_string(), cat.clone());
                    if let Some(category_key) = category_key {
                        let lang = language.as_deref().unwrap_or("english");
                        let shared_url = format!("https://store.akamai.steamstatic.com/public/javascript/applications/store/shared_{}-json.js", lang);
                        if let Ok(shared_resp) = reqwest::get(&shared_url).await {
                            if let Ok(shared_text) = shared_resp.text().await {
                                if let Some(start) = shared_text.find("JSON.parse('") {
                                    let json_start = start + "JSON.parse('".len();
                                    if let Some(end) = shared_text[json_start..].find("')") {
                                        let mut json_str = shared_text[json_start..json_start+end].to_string();
                                        json_str = json_str.replace(r"\'", "'").replace(r"\\", "\\");
                                        if let Ok(shared_json) = serde_json::from_str::<serde_json::Value>(&json_str) {
                                            if let Some(label_val) = shared_json.get(&category_key) {
                                                if let Some(label_str) = label_val.as_str() {
                                                    map.insert("steamdeck_category".to_string(), Value::String(label_str.to_string()));
                                                }
                                            }
                                            // Add labels to compatibility items
                                            for key in ["steamdeck_compatibility_items", "steamos_resolved_items"] {
                                                if let Some(Value::Array(items)) = map.get_mut(key) {
                                                    for item in items.iter_mut() {
                                                        if let Value::Object(obj) = item {
                                                            if let Some(Value::String(token)) = obj.get("token") {
                                                                if let Some(label_val) = shared_json.get(token) {
                                                                    if let Some(label_str) = label_val.as_str() {
                                                                        obj.insert("label".to_string(), Value::String(label_str.to_string()));
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            let size = Some(1);
            Json(ApiResponse::new(200, true, "OK".to_string(), size, data, chrono::Utc::now().to_rfc3339(), None))
        }
        None => Json(ApiResponse::new(500, false, "Malformed response from Steam".to_string(), None, None, chrono::Utc::now().to_rfc3339(), Some("appid entry missing".to_string()))),
    }
}

pub fn all_routes() -> Vec<Route> {
    routes![apps, app]
}
