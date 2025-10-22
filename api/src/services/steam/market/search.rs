use serde_json::Value;
use reqwest;
use urlencoding;

pub async fn fetch_search(
    appid: Option<String>,
    query: Option<String>,
    sort: Option<String>,
    page: Option<u32>,
) -> Result<(u64, Vec<Value>), String> {
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
    let resp = reqwest::get(&url)
        .await
        .map_err(|e| e.to_string())?;
        
    let json: Value = resp.json()
        .await
        .map_err(|e| e.to_string())?;

    if json.get("success") != Some(&Value::Bool(true)) {
        return Err("Invalid request or unsuccessful response".to_string());
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

    Ok((pagesize, processed_results))
}

use super::utils::process_icon_urls;
