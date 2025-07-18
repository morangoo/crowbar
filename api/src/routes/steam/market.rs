use rocket::get;
use rocket::response::content::RawHtml;
use reqwest;

#[get("/market/ping")]
pub async fn ping() -> RawHtml<String> {
    // Fetch the HTML from the Steam Market page
    match reqwest::get("https://steamcommunity.com/market/").await {
        Ok(resp) => match resp.text().await {
            Ok(html) => RawHtml(html),
            Err(_) => RawHtml("<h1>Error reading Steam Market response</h1>".to_string()),
        },
        Err(_) => RawHtml("<h1>Error fetching Steam Market</h1>".to_string()),
    }
}
