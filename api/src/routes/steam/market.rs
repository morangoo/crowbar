pub fn all_routes() -> Vec<rocket::Route> {
    routes![top, search]
}
use rocket::get;
use rocket::serde::{Serialize, Deserialize, json::Json};
use reqwest::Client;
use scraper::{Html, Selector, ElementRef};
use crate::response::ApiResponse;

use rocket::form::FromForm;

#[derive(FromForm, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct SearchParams {
    pub appid: Option<String>,
    pub query: Option<String>,
    pub orderby: Option<String>,
    pub page: Option<u32>,
}


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



fn extract_text<'a>(el: Option<ElementRef<'a>>) -> String {
    el.map(|e| e.text().collect::<Vec<_>>().join("").trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "-".to_string())
}

fn extract_price_and_currency(document: &Html, selector: &Selector) -> (f64, String) {
    let elem = document.select(selector)
        .find(|el| el.value().attr("class").map_or(false, |c| c.trim() == "normal_price"));
    match elem {
        Some(e) => {
            let price_str = e.text().collect::<String>().trim().to_string();
            let price = price_str.chars().filter(|c| c.is_ascii_digit() || *c == '.').collect::<String>().parse::<f64>().unwrap_or(0.0);
            let currency = e.value().attr("data-currency").unwrap_or("-").to_string();
            (price, currency)
        },
        None => (0.0, "-".to_string()),
    }
}

fn extract_qty(el: Option<ElementRef>) -> i32 {
    el.map(|e| {
        let qty_raw = e.text().collect::<Vec<_>>().join("").trim().to_string();
        qty_raw.replace(",", "").parse::<i32>().unwrap_or(0)
    }).unwrap_or(0)
}


#[get("/")]
pub async fn top() -> Json<ApiResponse<Vec<MarketResult>>> {
    let client = Client::new();
    let html = match client.get("https://steamcommunity.com/market/").send().await {
        Ok(resp) => match resp.text().await {
            Ok(text) => text,
            Err(e) => return Json(ApiResponse::error(500, "Erro ao ler HTML", &e.to_string())),
        },
        Err(e) => return Json(ApiResponse::error(500, "Erro na requisição", &e.to_string())),
    };
    let document = Html::parse_document(&html);
    let mut results = Vec::with_capacity(10);

    for i in 0..10 {
        let div_selector = Selector::parse(&format!("#result_{}", i)).unwrap();
        let img_selector = Selector::parse(&format!("img#result_{}_image", i)).unwrap();
        let name_selector = Selector::parse(&format!("span#result_{}_name", i)).unwrap();
        let game_selector = Selector::parse(&format!("#result_{} .market_listing_game_name", i)).unwrap();
        let price_selector = Selector::parse(&format!("#result_{} .normal_price", i)).unwrap();
        let qty_selector = Selector::parse(&format!("#result_{} .market_listing_num_listings_qty", i)).unwrap();
        let link_selector = Selector::parse(&format!("#resultlink_{}", i)).unwrap();

        let div = document.select(&div_selector).next();
        let appid = div.and_then(|el| el.value().attr("data-appid")).unwrap_or("-").to_string();
        let image = document.select(&img_selector).next().and_then(|el| el.value().attr("src")).unwrap_or("-").to_string();
        let item_link = document.select(&link_selector).next().and_then(|el| el.value().attr("href")).unwrap_or("-").to_string();
        let name = extract_text(document.select(&name_selector).next());
        let game = extract_text(document.select(&game_selector).next());
        let (price, currency) = extract_price_and_currency(&document, &price_selector);
        let qty = extract_qty(document.select(&qty_selector).next());

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

#[get("/search?<params..>")]
pub async fn search(params: SearchParams) -> Json<ApiResponse<Vec<MarketResult>>> {

    let appid = params.appid.as_deref().unwrap_or("");
    let query = params.query.as_deref().unwrap_or("");
    let orderby = params.orderby.as_deref().unwrap_or("default_desc");
    let page = params.page.unwrap_or(1);

    let mut url = String::from("https://steamcommunity.com/market/search?");
    let mut params_vec = Vec::new();
    if !appid.is_empty() {
        params_vec.push(format!("appid={}", appid));
    }
    if !query.is_empty() {
        params_vec.push(format!("q={}", query));
    }
    url.push_str(&params_vec.join("&"));
    url.push_str(&format!("#p{}_{}", page, orderby));

    println!("Requesting URL: {}", url);

    let client = Client::new();
    let html = match client.get(&url).send().await {
        Ok(resp) => match resp.text().await {
            Ok(text) => text,
            Err(e) => return Json(ApiResponse::error(500, "Erro ao ler HTML", &e.to_string())),
        },
        Err(e) => return Json(ApiResponse::error(500, "Erro na requisição", &e.to_string())),
    };

    let document = Html::parse_document(&html);
    let mut results = Vec::with_capacity(10);

    for i in 0..10 {
        let div_selector = Selector::parse(&format!("#result_{}", i)).unwrap();
        let img_selector = Selector::parse(&format!("img#result_{}_image", i)).unwrap();
        let name_selector = Selector::parse(&format!("span#result_{}_name", i)).unwrap();
        let game_selector = Selector::parse(&format!("#result_{} .market_listing_game_name", i)).unwrap();
        let price_selector = Selector::parse(&format!("#result_{} .normal_price", i)).unwrap();
        let qty_selector = Selector::parse(&format!("#result_{} .market_listing_num_listings_qty", i)).unwrap();
        let link_selector = Selector::parse(&format!("#resultlink_{}", i)).unwrap();

        let div = document.select(&div_selector).next();
        let appid = div.and_then(|el| el.value().attr("data-appid")).unwrap_or("-").to_string();
        let image = document.select(&img_selector).next().and_then(|el| el.value().attr("src")).unwrap_or("-").to_string();
        let item_link = document.select(&link_selector).next().and_then(|el| el.value().attr("href")).unwrap_or("-").to_string();
        let name = extract_text(document.select(&name_selector).next());
        let game = extract_text(document.select(&game_selector).next());
        let (price, currency) = extract_price_and_currency(&document, &price_selector);
        let qty = extract_qty(document.select(&qty_selector).next());

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