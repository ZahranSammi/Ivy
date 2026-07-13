use async_trait::async_trait;
use std::time::Duration;

use super::provider::LlmProvider;

const DEFAULT_ENDPOINT: &str = "https://generativelanguage.googleapis.com/v1beta/models";

pub struct GeminiProvider {
    client: reqwest::Client,
    api_key: String,
    model: String,
    endpoint: String,
}

impl GeminiProvider {
    pub fn new(api_key: String, model: String, endpoint: Option<String>) -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .expect("reqwest client"),
            api_key,
            model,
            endpoint: endpoint.unwrap_or_else(|| DEFAULT_ENDPOINT.to_string()),
        }
    }
}

fn extract_text(body: &serde_json::Value) -> Result<String, anyhow::Error> {
    body["candidates"][0]["content"]["parts"][0]["text"]
        .as_str()
        .map(str::to_string)
        .ok_or_else(|| anyhow::anyhow!("unexpected Gemini response shape: {body}"))
}

#[async_trait]
impl LlmProvider for GeminiProvider {
    async fn generate(&self, prompt: &str) -> Result<String, anyhow::Error> {
        let url = format!(
            "{}/{}:generateContent?key={}",
            self.endpoint, self.model, self.api_key
        );
        let res = self
            .client
            .post(&url)
            .json(&serde_json::json!({
                "contents": [{ "parts": [{ "text": prompt }] }]
            }))
            .send()
            .await?;

        if !res.status().is_success() {
            anyhow::bail!(
                "Gemini API error {}: {}",
                res.status(),
                res.text().await.unwrap_or_default()
            );
        }

        extract_text(&res.json::<serde_json::Value>().await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_text_from_gemini_response() {
        let body = serde_json::json!({
            "candidates": [{ "content": { "parts": [{ "text": "hello" }] } }]
        });
        assert_eq!(extract_text(&body).unwrap(), "hello");
    }

    #[test]
    fn errors_on_unexpected_shape() {
        let body = serde_json::json!({ "candidates": [] });
        assert!(extract_text(&body).is_err());
    }
}
