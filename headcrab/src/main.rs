use std::{thread, time::Duration};
use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize)]
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
async fn main() -> Result<(), Error> {
    loop {
        let resp = reqwest::get("https://api.steampowered.com/ISteamApps/GetAppList/v2/")
            .await?
            .json::<ApiResponse>()
            .await?;

        for app in resp.applist.apps.iter() {
            println!("AppID: {}, Nome: {}", app.appid, app.name);
        }

        println!("Tarefa finalizada. Aguardando 4 horas para reiniciar...");
        thread::sleep(Duration::from_secs(4 * 60 * 60)); // 4 horas
    }
}
