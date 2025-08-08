use rocket::serde::json::Json;
use crate::response::ApiResponse;
use rocket::{get, Route};
use serde_json::Value;

#[get("/games")]
pub async fn games() -> Json<ApiResponse<Value>> {
    Json(ApiResponse::new(
        200,
        true,
        "OK".to_string(),
        None,
        None,
        chrono::Utc::now().to_rfc3339(),
        None,
    ))
}

#[get("/game?<appid>")]
pub async fn game(appid: Option<u32>) -> Json<ApiResponse<Value>> {
    let appid = match appid {
        Some(id) => id,
        None => {
            return Json(ApiResponse::new(
                400,
                false,
                "Missing appid".to_string(),
                None,
                None,
                chrono::Utc::now().to_rfc3339(),
                Some("appid missing".to_string()),
            ));
        }
    };
    let url = format!("https://store.steampowered.com/api/appdetails?appids={}", appid);
    let resp = match reqwest::get(&url).await {
        Ok(r) => r,
        Err(e) => return Json(ApiResponse::new(
            500,
            false,
            "Error making request".to_string(),
            None,
            None,
            chrono::Utc::now().to_rfc3339(),
            Some(e.to_string()),
        )),
    };
    let json: Value = match resp.json().await {
        Ok(j) => j,
        Err(e) => return Json(ApiResponse::new(
            500,
            false,
            "Error reading JSON response".to_string(),
            None,
            None,
            chrono::Utc::now().to_rfc3339(),
            Some(e.to_string()),
        )),
    };
    let entry = json.get(appid.to_string()).and_then(|v| v.as_object());
    match entry {
        Some(obj) => {
            let success = obj.get("success").and_then(|v| v.as_bool()).unwrap_or(false);
            if !success {
                return Json(ApiResponse::new(
                    404,
                    false,
                    "App not found or invalid appid".to_string(),
                    None,
                    None,
                    chrono::Utc::now().to_rfc3339(),
                    Some("success == false".to_string()),
                ));
            }
            let data = obj.get("data").cloned();
            let size = Some(1);
            Json(ApiResponse::new(
                200,
                true,
                "OK".to_string(),
                size,
                data,
                chrono::Utc::now().to_rfc3339(),
                None,
            ))
        }
        None => Json(ApiResponse::new(
            500,
            false,
            "Malformed response from Steam".to_string(),
            None,
            None,
            chrono::Utc::now().to_rfc3339(),
            Some("appid entry missing".to_string()),
        )),
    }
}

pub fn all_routes() -> Vec<Route> {
    routes![games, game]
}
