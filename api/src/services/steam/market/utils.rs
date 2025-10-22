use serde_json::Value;

pub fn process_icon_urls(obj: &mut serde_json::Map<String, Value>) {
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

pub fn make_icon_url(s: &str) -> String {
    format!("https://community.fastly.steamstatic.com/economy/image/{}", s)
}
