impl<T> ApiResponse<T> {
    pub fn new(
        code: u16,
        success: bool,
        message: String,
        size: Option<u64>,
        data: Option<T>,
        timestamp: String,
        error: Option<String>,
    ) -> Self {
        Self {
            code,
            success,
            message,
            size,
            data,
            timestamp,
            error,
            apiversion: "v0.0.1cb",
        }
    }
}
use rocket::serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub apiversion: &'static str,
    pub code: u16,
    pub success: bool,
    pub message: String,
    pub size: Option<u64>,
    pub data: Option<T>,
    pub timestamp: String,
    pub error: Option<String>,
}