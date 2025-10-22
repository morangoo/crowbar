use serde_json::Value;
use scraper::{Html, Selector};
use reqwest;
use urlencoding;

pub async fn fetch_item(appid: &str, marketname: &str) -> Result<Value, String> {
    if appid.is_empty() || marketname.is_empty() {
        return Err("Missing appid or marketname".to_string());
    }

    let encoded_name = urlencoding::encode(marketname);
    let url = format!(
        "https://steamcommunity.com/market/listings/{}/{}",
        appid, encoded_name
    );

    let resp = reqwest::get(&url)
        .await
        .map_err(|e| e.to_string())?;

    let html = resp.text()
        .await
        .map_err(|e| e.to_string())?;

    let document = Html::parse_document(&html);
    let selector = Selector::parse("script").unwrap();

    for script in document.select(&selector) {
        let script_text = script.text().collect::<Vec<_>>().join("");
        if let Some(idx) = script_text.find("g_rgAssets = ") {
            let json_start = idx + "g_rgAssets = ".len();
            if let Some(end_idx) = script_text[json_start..].find(";") {
                let json_str = &script_text[json_start..json_start+end_idx];
                let result = serde_json::from_str::<Value>(json_str)
                    .map_err(|e| e.to_string())?;

                let mut current = &result;
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

                return Ok(obj);
            }
        }
    }

    Err("Asset information not found".to_string())
}

use super::utils::process_icon_urls;
