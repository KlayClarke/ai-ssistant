use std::{env, io::ErrorKind};

use async_channel::Sender;
use futures::StreamExt;
use reqwest::{Client, Error, Response};

use crate::api_types::{APIResponse, Content, Usage};

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

    pub async fn send_chat_message(&self, content: &str, sender: Sender<Result<APIResponse, Error>>) -> Result<Response, Error> {
        let url = "https://api.anthropic.com/v1/messages";
        let json_data = serde_json::json!({
            "model": "claude-3-opus-20240229",
            "max_tokens": 1024,
            "messages": [{"role": "user", "content": content}],
            "stream": true
        });
        let response = self.client
                .post(url)
                .header("x-api-key", &self.api_key)
                .header("anthropic-version", "2023-06-01")
                .json(&json_data)
                .send()
                .await;
        response
    }
}