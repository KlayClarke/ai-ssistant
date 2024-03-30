use reqwest::{Client, Error, Response};

use crate::chat_object::ChatData;

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

    pub async fn send_chat_message(&self, conversation: &Vec<ChatData>) -> Result<Response, Error> {
        println!("api_client line 19 conversation print: {:?}", conversation);
        let api_key = self.api_key.clone();
        let url = "https://api.anthropic.com/v1/messages";
        let json_data = serde_json::json!({
            "model": "claude-3-opus-20240229",
            "max_tokens": 256,
            "messages": conversation.clone(),
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