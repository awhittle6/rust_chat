use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum ApplicationError {
    ServerError {code: u32, message: String}
}

#[derive(Debug, Serialize, Clone)]
pub struct ChatMessage {
    pub from: String,
    pub message: String,
    pub timestamp: i64,
}