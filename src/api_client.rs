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
        let test_user_chat = ChatData {
            role: "user".to_string(),
            content: "what is the first positive number?".to_string()
        };

        let test_assistant_chat = ChatData {
            role: "assistant".to_string(),
            content: "0".to_string()
        };

        let test_user_response = ChatData {
            role: "user".to_string(),
            content: "what is the number after that one?".to_string()
        };

        let api_key = self.api_key.clone();
        let url = "https://api.anthropic.com/v1/messages";
        let json_data = serde_json::json!({
            "model": "claude-3-opus-20240229",
            "max_tokens": 256,
            "messages": [test_user_chat, test_assistant_chat, test_user_response],
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