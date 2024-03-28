use serde::{Deserialize, Serialize};

// Claude API types
#[derive(Serialize, Deserialize, Debug)]
pub struct Content {
    text: String,
    #[serde(rename="type")]
    type_: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Usage {
    input_tokens: u32,
    output_tokens: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct APIResponse {
    id: String,
    #[serde(rename="type")]
    type_: String,
    role: String,
    content: Vec<Content>,
    model: String,
    stop_reason: String,
    stop_sequence: Option<String>,
    usage: Usage,
}