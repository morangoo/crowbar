use rocket::get;
use rocket::response::content::RawHtml;
use reqwest;
use scraper::{Html, Selector};

#[get("/")]
pub async fn ping() -> RawHtml<String> {
    // Fetch the HTML from the Steam Market page with Accept-Language header
    let client = reqwest::Client::new();
    let request = client
        .get("https://steamcommunity.com/market/");
        //.header("Accept-Language", "pt-PT,pt;q=0.7");
    match request.send().await {
        Ok(resp) => match resp.text().await {
            Ok(html) => {
                let document = Html::parse_document(&html);
                let mut rows = String::new();
                for i in 0..10 {
                    let img_selector = Selector::parse(&format!("img#result_{}_image", i)).unwrap();
                    let name_selector = Selector::parse(&format!("span#result_{}_name", i)).unwrap();
                    let img_html = if let Some(img) = document.select(&img_selector).next() {
                        if let Some(src) = img.value().attr("src") {
                            format!("<img src=\"{}\" width=\"64\" />", src)
                        } else {
                            "-".to_string()
                        }
                    } else {
                        "-".to_string()
                    };
                    let name_html = if let Some(name) = document.select(&name_selector).next() {
                        let name_text = name.text().collect::<Vec<_>>().join("").trim().to_string();
                        name_text
                    } else {
                        "-".to_string()
                    };
                    rows.push_str(&format!("<tr><td>{}</td><td>{}</td></tr>", img_html, name_html));
                }
                let table = format!(
                    "<table border=1 cellpadding=6><tr><th>Image</th><th>Name</th></tr>{}</table>",
                    rows
                );
                RawHtml(table)
            },
            Err(_) => RawHtml("<h1>Error reading Steam Market response</h1>".to_string()),
        },
        Err(_) => RawHtml("<h1>Error fetching Steam Market</h1>".to_string()),
    }
}
