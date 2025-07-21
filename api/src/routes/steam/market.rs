
use rocket::get;
use rocket::serde::{Serialize, json::Json};
use reqwest;
use scraper::{Html, Selector};
use crate::response::ApiResponse;

#[derive(Serialize)]
pub struct MarketResult {
    pub image: String,
    pub name: String,
    pub game: String,
    pub price: f64,
    pub currency: String,
    pub qty: i32,
    pub appid: String,
    pub item_link: String,
}


fn extract_text(document: &Html, selector: &Selector) -> String {
    document.select(selector)
        .next()
        .map(|el| el.text().collect::<Vec<_>>().join("").trim().to_string())
        .unwrap_or_else(|| "-".to_string())
}

fn extract_price_and_currency(document: &Html, selector: &Selector) -> (f64, String) {
    if let Some(price_elem) = document.select(selector).find(|el| {
        if let Some(class) = el.value().attr("class") {
            class.trim() == "normal_price"
        } else {
            false
        }
    }) {
        let price_raw = price_elem.text().collect::<Vec<_>>().join("").trim().to_string();
        let price_clean: String = price_raw.chars().filter(|c| c.is_ascii_digit() || *c == '.').collect();
        let price = price_clean.parse::<f64>().unwrap_or(0.0);
        let currency = price_elem.value().attr("data-currency").unwrap_or("-").to_string();
        (price, currency)
    } else {
        (0.0, "-".to_string())
    }
}

fn extract_qty(document: &Html, selector: &Selector) -> i32 {
    document.select(selector)
        .next()
        .map(|el| {
            let qty_raw = el.text().collect::<Vec<_>>().join("").trim().to_string();
            let qty_clean: String = qty_raw.replace(",", "");
            qty_clean.parse::<i32>().unwrap_or(0)
        })
        .unwrap_or(0)
}

#[get("/")]
pub async fn top() -> Json<ApiResponse<Vec<MarketResult>>> {
    let client = reqwest::Client::new();
    let request = client.get("https://steamcommunity.com/market/");
    let response = match request.send().await {
        Ok(resp) => resp,
        Err(e) => return Json(ApiResponse::error(500, "Request error", &e.to_string())),
    };
    let html = match response.text().await {
        Ok(html) => html,
        Err(e) => return Json(ApiResponse::error(500, "Error reading HTML", &e.to_string())),
    };
    let document = Html::parse_document(&html);
    let mut results = Vec::with_capacity(10);
    for i in 0..10 {
        let img_selector = Selector::parse(&format!("img#result_{}_image", i)).unwrap();
        let name_selector = Selector::parse(&format!("span#result_{}_name", i)).unwrap();
        let game_selector = Selector::parse(&format!("#result_{} .market_listing_game_name", i)).unwrap();
        let price_selector = Selector::parse(&format!("#result_{} .normal_price", i)).unwrap();
        let qty_selector = Selector::parse(&format!("#result_{} .market_listing_num_listings_qty", i)).unwrap();

        let div_selector = Selector::parse(&format!("#result_{}", i)).unwrap();
        let appid = document.select(&div_selector)
            .next()
            .and_then(|el| el.value().attr("data-appid"))
            .unwrap_or("-").to_string();

        let image = document.select(&img_selector)
            .next()
            .and_then(|el| el.value().attr("src"))
            .unwrap_or("-").to_string();

        let link_selector = Selector::parse(&format!("#resultlink_{}", i)).unwrap();
        let item_link = document.select(&link_selector)
            .next()
            .and_then(|el| el.value().attr("href"))
            .unwrap_or("-").to_string();

        let name = extract_text(&document, &name_selector);
        let game = extract_text(&document, &game_selector);
        let (price, currency) = extract_price_and_currency(&document, &price_selector);
        let qty = extract_qty(&document, &qty_selector);

        results.push(MarketResult {
            image,
            name,
            game,
            price,
            currency,
            qty,
            appid,
            item_link,
        });
    }
    Json(ApiResponse::success(results, "Data retrieved successfully"))
}