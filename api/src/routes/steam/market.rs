
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
    pub price: String,
    pub currency: String,
    pub qty: String,
}

#[get("/top")]
pub async fn top() -> Json<ApiResponse<Vec<MarketResult>>> {
    let client = reqwest::Client::new();
    let request = client.get("https://steamcommunity.com/market/");
    match request.send().await {
        Ok(resp) => match resp.text().await {
            Ok(html) => {
                let document = Html::parse_document(&html);
                let mut results = Vec::new();
    for i in 0..10 {
        let img_selector = Selector::parse(&format!("img#result_{}_image", i)).unwrap();
        let name_selector = Selector::parse(&format!("span#result_{}_name", i)).unwrap();
        let game_selector = Selector::parse(&format!("#result_{} .market_listing_game_name", i)).unwrap();
        let price_selector = Selector::parse(&format!("#result_{} .normal_price", i)).unwrap();
        let qty_selector = Selector::parse(&format!("#result_{} .market_listing_num_listings_qty", i)).unwrap();

        let image = if let Some(img) = document.select(&img_selector).next() {
            img.value().attr("src").unwrap_or("-").to_string()
        } else {
            "-".to_string()
        };
        let name = if let Some(name) = document.select(&name_selector).next() {
            name.text().collect::<Vec<_>>().join("").trim().to_string()
        } else {
            "-".to_string()
        };
        let game = if let Some(game) = document.select(&game_selector).next() {
            game.text().collect::<Vec<_>>().join("").trim().to_string()
        } else {
            "-".to_string()
        };
        let (price, currency) = if let Some(price_elem) = document.select(&price_selector).find(|el| {
            if let Some(class) = el.value().attr("class") {
                class.trim() == "normal_price"
            } else {
                false
            }
        }) {
            let price = price_elem.text().collect::<Vec<_>>().join("").trim().to_string();
            let currency = price_elem.value().attr("data-currency").unwrap_or("-").to_string();
            (price, currency)
        } else {
            ("-".to_string(), "-".to_string())
        };
        let qty = if let Some(qty_elem) = document.select(&qty_selector).next() {
            qty_elem.text().collect::<Vec<_>>().join("").trim().to_string()
        } else {
            "-".to_string()
        };
        results.push(MarketResult { image, name, game, price, currency, qty });
    }
                Json(ApiResponse::success(results, "Data retrieved successfully"))
            },
            Err(e) => Json(ApiResponse::error(500, "Error reading HTML", &e.to_string())),
        },
        Err(e) => Json(ApiResponse::error(500, "Request error", &e.to_string())),
    }
}