use std::{thread, time::Duration};
use serde::{Deserialize, Serialize};
use mongodb::{Client, options::ClientOptions, bson::doc};
use reqwest;

#[derive(Serialize, Deserialize)]
struct App {
    appid: u32,
    name: String,
}

#[derive(Deserialize)]
struct AppList {
    apps: Vec<App>,
}

#[derive(Deserialize)]
struct ApiResponse {
    applist: AppList,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let client_uri = "mongodb://localhost:27017";
    let client_options = ClientOptions::parse(client_uri).await?;
    let client = Client::with_options(client_options)?;
    let db = client.database("crate");
    let collection = db.collection::<App>("apps");

    loop {
        let resp = reqwest::get("https://api.steampowered.com/ISteamApps/GetAppList/v2/")
            .await?
            .json::<ApiResponse>()
            .await?;

        for app in &resp.applist.apps {
            collection.insert_one(app, None).await?;
            tokio::time::sleep(std::time::Duration::from_secs(2)).await; // 2 seconds
        }

        thread::sleep(Duration::from_secs(4 * 60 * 60)); // 4 hours
    }
}
