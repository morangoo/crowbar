use rocket::serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub code: u16,
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
    pub timestamp: String,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T, message: &str) -> Self {
        Self {
            code: 200,
            success: true,
            message: message.to_string(),
            data: Some(data),
            timestamp: chrono::Utc::now().to_rfc3339(),
            error: None,
        }
    }

    pub fn error(code: u16, message: &str, error: &str) -> Self {
        Self {
            code,
            success: false,
            message: message.to_string(),
            data: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
            error: Some(error.to_string()),
        }
    }
}