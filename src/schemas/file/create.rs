use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateFileRequest {
    pub filename: String,
    pub content: Option<String>,
}

