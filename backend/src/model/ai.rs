use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AiChat {
    pub prompt: String,
    pub history: Vec<Message>,
}
