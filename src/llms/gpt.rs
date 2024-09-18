use reqwest::Client;
use crate::traits::LLMRequest;

pub struct GPT4Model {
    client: Client,
    api_key: String,
}

impl GPT4Model {
    pub fn new() -> Self {
        let api_key = "".to_string(); // Replace with your actual API key
        GPT4Model {
            client: Client::new(),
            api_key,
        }
    }

    pub async fn req(&self, prompt: &str) -> Result<String, reqwest::Error> {
        let url = "https://api.openai.com/v1/completions";
        let response = self.client.post(url)
        .bearer_auth(&self.api_key)
        .json(&serde_json::json!({
            "model": "gpt-3.5",
            "prompt": prompt,
            "max_tokens": 100,
        }))
        .send()
        .await?
        .text()
        .await?;
        Ok(response)
    }
}

