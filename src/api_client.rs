use async_channel::Sender;
use reqwest::{Client, Error, Response};

pub struct APIClient {
    client: Client,
    api_key: String,
}

impl APIClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    pub async fn send_chat_message(&self, content: &str) -> Result<Response, Error> {
        let api_key = self.api_key.clone();
        let url = "https://api.anthropic.com/v1/messages";
        let json_data = serde_json::json!({
            "model": "claude-3-opus-20240229",
            "max_tokens": 1024,
            "messages": [{"role": "user", "content": content}],
        });
        let response = self.client
                .post(url)
                .header("x-api-key", api_key)
                .header("anthropic-version", "2023-06-01")
                .json(&json_data)
                .send()
                .await;
        response
    }
}