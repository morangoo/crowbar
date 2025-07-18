
use rocket::get;
use rocket::serde::{Serialize, json::Json};
use reqwest;
use scraper::{Html, Selector};
use crate::response::ApiResponse;

#[derive(Serialize)]
pub struct MarketResult {
    pub image: String,
    pub name: String,
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
                    results.push(MarketResult { image, name });
                }
                Json(ApiResponse::success(results, "Data retrieved successfully"))
            },
            Err(e) => Json(ApiResponse::error(500, "Error reading HTML", &e.to_string())),
        },
        Err(e) => Json(ApiResponse::error(500, "Request error", &e.to_string())),
    }
}