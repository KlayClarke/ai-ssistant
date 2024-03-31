use serde::{Deserialize, Serialize};

// Claude API request types
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApiRequest {
    pub role: String,
    #[serde(flatten)]
    pub content: RequestContent,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum RequestContent {
    Text(String),
    Blocks(Vec<Block>),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum Block {
    Text { text: String },
    Image { source: ImageSource },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ImageSource {
    #[serde(rename = "type")]
    pub source_type: String,
    pub media_type: String,
    pub data: String,
}

// Claude API response types
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