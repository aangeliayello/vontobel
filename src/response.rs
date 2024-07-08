use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Response<T> {
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> Response<T> {
    pub fn error(error_msg: &str) -> Self {
        Self { data: None, error: Some(error_msg.to_string()) }
    }
}