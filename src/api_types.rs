use serde::{Deserialize, Serialize};

// Claude API types
#[derive(Serialize, Deserialize, Debug)]
pub struct Content {
    pub text: String,
    #[serde(rename="type")]
    pub type_: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Usage {
    pub input_tokens: u32,
    pub output_tokens: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct APIResponse {
    pub id: String,
    #[serde(rename="type")]
    pub type_: String,
    pub role: String,
    pub content: Vec<Content>,
    pub model: String,
    pub stop_reason: String,
    pub stop_sequence: Option<String>,
    pub usage: Usage,
}