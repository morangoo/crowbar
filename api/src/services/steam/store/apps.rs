use serde_json::Value;
use reqwest::Client;
use scraper::{Html, Selector};

pub async fn fetch_apps(
    term: Option<String>,
    page: Option<u32>,
    count: Option<u32>,
    cc: Option<String>,
    language: Option<String>,
    tags: Option<Vec<u32>>,
) -> Result<Value, String> {
    let mut url = String::from("https://store.steampowered.com/search/results/?norender=1&ignore_preferences=1");
    if let Some(q) = term {
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
    if let Some(ref tag_list) = tags {
        if !tag_list.is_empty() {
            let tags_param = tag_list.iter().map(|t| t.to_string()).collect::<Vec<_>>().join("%2C");
            url.push_str(&format!("&tags={}", tags_param));
        }
    }

    let client = Client::new();
    let resp = client
        .get(&url)
        .header(
            reqwest::header::COOKIE,
            "wants_mature_content=1; lastagecheckage=1-January-2000; birthtime=946684801",
        )
        .send()
        .await
        .map_err(|e| format!("request error: {}", e))?;

    let html = resp.text().await.map_err(|e| format!("read error: {}", e))?;
    let document = Html::parse_document(&html);
    let selector = Selector::parse("a.search_result_row").map_err(|e| format!("selector error: {}", e))?;
    let mut results = Vec::new();
    for game in document.select(&selector) {
        let mut obj = serde_json::Map::new();
        if let Some(appid) = game.value().attr("data-ds-appid").and_then(|v| v.parse::<u32>().ok()) {
            obj.insert("appid".to_string(), Value::Number(appid.into()));
            obj.insert("img_large".to_string(), Value::String(format!("https://shared.fastly.steamstatic.com/store_item_assets/steam/apps/{}/header.jpg", appid)));
            obj.insert("cover".to_string(), Value::String(format!("https://cdn.cloudflare.steamstatic.com/steam/apps/{}/library_600x900.jpg", appid)));
        }
        obj.insert(
            "title".to_string(),
            game.select(&Selector::parse("span.title").unwrap()).next().map(|e| e.text().collect::<String>()).unwrap_or_default().into(),
        );
        obj.insert("url".to_string(), game.value().attr("href").map(|s| Value::String(s.to_string())).unwrap_or(Value::Null));
        obj.insert("img".to_string(), game.select(&Selector::parse(".search_capsule img").unwrap()).next().and_then(|e| e.value().attr("src")).map(|s| Value::String(s.to_string())).unwrap_or(Value::Null));

        // Price and discount
        let price_final = game.select(&Selector::parse(".discount_final_price").unwrap()).next().map(|e| e.text().collect::<String>());
        let mut price_original = game.select(&Selector::parse(".discount_original_price").unwrap()).next().map(|e| e.text().collect::<String>());
        if price_original.is_none() {
            price_original = price_final.clone();
        }
        obj.insert("price_final".to_string(), price_final.clone().map(Value::String).unwrap_or(Value::Null));
        obj.insert("price_original".to_string(), price_original.clone().map(Value::String).unwrap_or(Value::Null));

        fn extract_price_num(price: &str) -> Option<u64> {
            let digits: String = price.chars().filter(|c| c.is_ascii_digit()).collect();
            if digits.is_empty() { None } else { digits.parse::<u64>().ok() }
        }

        if let Some(ref pf) = price_final {
            if let Some(num) = extract_price_num(pf) {
                obj.insert("price_final_num".to_string(), Value::Number(num.into()));
            }
        }
        if let Some(po) = &price_original {
            if let Some(num) = extract_price_num(po) {
                obj.insert("price_original_num".to_string(), Value::Number(num.into()));
            }
        }

        let discount_pct = game.select(&Selector::parse(".discount_pct").unwrap()).next().map(|e| e.text().collect::<String>());
        let discount_value = discount_pct.as_ref().and_then(|d| d.strip_prefix('-').and_then(|s| s.strip_suffix('%')).and_then(|s| s.parse::<u32>().ok())).unwrap_or(0);
        obj.insert("discount_pct".to_string(), Value::String(discount_pct.clone().unwrap_or("0%".to_string())));
        obj.insert("discounted".to_string(), Value::Bool(discount_pct.is_some()));
        obj.insert("discount".to_string(), Value::Number(discount_value.into()));
        obj.insert("bundle_discount".to_string(), game.select(&Selector::parse(".discount_block").unwrap()).next().and_then(|e| e.value().attr("data-bundlediscount")).map(|s| Value::String(s.to_string())).unwrap_or(Value::Null));

        obj.insert("released".to_string(), game.select(&Selector::parse(".search_released").unwrap()).next().map(|e| Value::String(e.text().collect())).unwrap_or(Value::Null));
        obj.insert("review".to_string(), game.select(&Selector::parse(".search_review_summary").unwrap()).next().and_then(|e| e.value().attr("data-tooltip-html")).map(|s| Value::String(s.to_string())).unwrap_or(Value::Null));

        let mut platforms = Vec::new();
        for p in game.select(&Selector::parse(".platform_img").unwrap()) {
            if let Some(class) = p.value().attr("class") {
                if class.contains("win") { platforms.push(Value::String("windows".to_string())); }
                if class.contains("mac") { platforms.push(Value::String("mac".to_string())); }
                if class.contains("linux") { platforms.push(Value::String("linux".to_string())); }
            }
        }
        if !platforms.is_empty() { obj.insert("platforms".to_string(), Value::Array(platforms)); }

        for &(key, attr) in &[ ("tags", "data-ds-tagids"), ("descids", "data-ds-descids"), ("crtrids", "data-ds-crtrids"), ("itemkey", "data-ds-itemkey"), ("steamdeck", "data-ds-steam-deck-compat-handled") ] {
            if let Some(val) = game.value().attr(attr) {
                obj.insert(key.to_string(), Value::String(val.to_string()));
            }
        }
        results.push(Value::Object(obj));
    }

    Ok(Value::Array(results))
}
