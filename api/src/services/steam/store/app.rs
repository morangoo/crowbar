use serde_json::Value;
use reqwest;
use scraper::{Html, Selector};
use crate::maps::steamdeck_compat_map;

fn extract_input_value(document: &Html, id: &str) -> Option<u64> {
    let selector = Selector::parse(&format!("input#{}", id)).ok()?;
    document.select(&selector).next()?.value().attr("value")?.parse::<u64>().ok()
}

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

pub async fn fetch_app(appid: u32, language: Option<String>, cc: Option<String>) -> Result<Value, String> {
    let mut url = format!("https://store.steampowered.com/api/appdetails?appids={}", appid);
    if let Some(ref lang) = language {
        url.push_str(&format!("&l={}", urlencoding::encode(lang)));
    }
    if let Some(country) = cc {
        url.push_str(&format!("&cc={}", urlencoding::encode(&country)));
    }

    let resp = reqwest::get(&url).await.map_err(|e| format!("request error: {}", e))?;
    let json: Value = resp.json().await.map_err(|e| format!("json error: {}", e))?;
    let entry = json.get(&appid.to_string()).and_then(|v| v.as_object());
    let mut data = entry.and_then(|obj| obj.get("data").cloned());

    // fetch html for extra info
    let app_url = format!("https://store.steampowered.com/app/{}", appid);
    let client = reqwest::Client::new();
    let html_text = match client
        .get(&app_url)
        .header(reqwest::header::COOKIE, "wants_mature_content=1; lastagecheckage=1-January-2000; birthtime=946684801")
        .send()
        .await
    {
        Ok(html_resp) => match html_resp.text().await {
            Ok(t) => t,
            Err(_) => String::new(),
        },
        Err(_) => String::new(),
    };

    let (positive_reviews, total_reviews, resolved_category, category_key, app_categories, app_tags) = {
        let document = Html::parse_document(&html_text);
        let positive_reviews = extract_input_value(&document, "review_summary_num_positive_reviews");
        let total_reviews = extract_input_value(&document, "review_summary_num_reviews");

        let mut resolved_category: Option<serde_json::Value> = None;
        let mut category_key: Option<String> = None;
        if let Some(div) = document.select(&Selector::parse("div#application_config").unwrap()).next() {
            if let Some(deckcompat) = div.value().attr("data-deckcompatibility") {
                if let Ok(deck_json) = serde_json::from_str::<serde_json::Value>(deckcompat) {
                    if let Some(cat) = deck_json.get("resolved_category") {
                        resolved_category = Some(cat.clone());
                        if let Some(cat_num) = cat.as_u64() {
                            let compat_map = steamdeck_compat_map::steamdeck_compatibility_map();
                            if let Some(key) = compat_map.get(&(cat_num as u8)) {
                                category_key = Some(key.to_string());
                            }
                        }
                    }
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

        let mut app_categories = Vec::new();
        if let Some(category_block) = document.select(&Selector::parse("div#category_block").unwrap()).next() {
            let feature_list_selector = Selector::parse("div.game_area_features_list_ctn a.game_area_details_specs_ctn").unwrap();
            for a in category_block.select(&feature_list_selector) {
                let icon = a.select(&Selector::parse("img.category_icon").unwrap()).next().and_then(|img| img.value().attr("src")).map(|s| Value::String(s.to_string())).unwrap_or(Value::Null);
                let href = a.value().attr("href").unwrap_or("");
                let category = href.split("category2=").nth(1).and_then(|s| s.split('&').next()).and_then(|s| s.parse::<u64>().ok()).map(|n| Value::Number(n.into())).unwrap_or(Value::Null);
                let label = a.select(&Selector::parse("div.label").unwrap()).next().map(|l| Value::String(l.text().collect::<String>())).unwrap_or(Value::Null);
                let mut obj = serde_json::Map::new();
                obj.insert("icon".to_string(), icon);
                obj.insert("category".to_string(), category);
                obj.insert("label".to_string(), label);
                app_categories.push(Value::Object(obj));
            }
        }

        let mut app_tags = Vec::new();
        if let Some(js_start) = html_text.find("InitAppTagModal(") {
            let js_sub = &html_text[js_start..];
            if let Some(arr_start) = js_sub.find('[') {
                let mut depth = 0;
                let mut arr_end = None;
                for (i, c) in js_sub[arr_start..].char_indices() {
                    match c {
                        '[' => depth += 1,
                        ']' => {
                            depth -= 1;
                            if depth == 0 {
                                arr_end = Some(arr_start + i + 1);
                                break;
                            }
                        }
                        _ => {}
                    }
                }
                if let Some(arr_end) = arr_end {
                    let arr_str = &js_sub[arr_start..arr_end];
                    if let Ok(tags_json) = serde_json::from_str::<serde_json::Value>(arr_str) {
                        if let Value::Array(tags) = tags_json {
                            for tag in tags {
                                app_tags.push(tag);
                            }
                        }
                    }
                }
            }
        }

        (positive_reviews, total_reviews, resolved_category, category_key, app_categories, app_tags)
    };

    // Fetch current players
    let stats_url = format!("https://api.steampowered.com/ISteamUserStats/GetNumberOfCurrentPlayers/v1/?appid={}", appid);
    let current_players = match reqwest::get(&stats_url).await {
        Ok(resp) => match resp.json::<serde_json::Value>().await {
            Ok(json) => json.get("response").and_then(|r| r.get("player_count")).and_then(|v| v.as_u64()).map(|num| Value::Number(num.into())),
            Err(_) => None,
        },
        Err(_) => None,
    };

    if let Some(Value::Object(ref mut map)) = data {
        map.insert("cover_image".to_string(), Value::String(format!("https://cdn.cloudflare.steamstatic.com/steam/apps/{}/library_600x900.jpg", appid)));
        if let (Some(pos), Some(total)) = (positive_reviews, total_reviews) {
            map.insert("positive_reviews".to_string(), Value::Number(pos.into()));
            map.insert("total_reviews".to_string(), Value::Number(total.into()));
            let percentage = if total > 0 { let pct = (pos as f64) * 100.0 / (total as f64); format!("{:.2}", pct) } else { "0.00".to_string() };
            map.insert("positive_reviews_percentage".to_string(), Value::String(percentage));
        }
        map.insert("current_players".to_string(), current_players.unwrap_or(Value::Null));

        let mut enriched_categories = app_categories;
        if let Some(Value::Array(categories)) = map.get("categories") {
            for cat_obj in &mut enriched_categories {
                if let Value::Object(ref mut obj) = cat_obj {
                    if let Some(Value::Number(cat_id)) = obj.get("category") {
                        if let Some(id) = cat_id.as_u64() {
                            if let Some(Value::Object(api_cat)) = categories.iter().find(|c| c.get("id").and_then(|v| v.as_u64()) == Some(id)) {
                                if let Some(Value::String(desc)) = api_cat.get("description") {
                                    obj.insert("description".to_string(), Value::String(desc.clone()));
                                }
                            }
                        }
                    }
                }
            }
        }
        map.insert("app_categories".to_string(), Value::Array(enriched_categories));
        map.remove("categories");

        map.insert("app_tags".to_string(), Value::Array(app_tags));
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

    match data {
        Some(d) => Ok(d),
        None => Err("Malformed response from Steam".to_string()),
    }
}
