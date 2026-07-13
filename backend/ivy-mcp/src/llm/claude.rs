use async_trait::async_trait;
use std::time::Duration;

use super::provider::LlmProvider;

const DEFAULT_ENDPOINT: &str = "https://api.anthropic.com/v1/messages";
const ANTHROPIC_VERSION: &str = "2023-06-01";

pub struct ClaudeProvider {
    client: reqwest::Client,
    api_key: String,
    model: String,
    endpoint: String,
}

impl ClaudeProvider {
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
    body["content"][0]["text"]
        .as_str()
        .map(str::to_string)
        .ok_or_else(|| anyhow::anyhow!("unexpected Claude response shape: {body}"))
}

#[async_trait]
impl LlmProvider for ClaudeProvider {
    async fn generate(&self, prompt: &str) -> Result<String, anyhow::Error> {
        let res = self
            .client
            .post(&self.endpoint)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", ANTHROPIC_VERSION)
            .json(&serde_json::json!({
                "model": self.model,
                "max_tokens": 1024,
                "messages": [{ "role": "user", "content": prompt }],
            }))
            .send()
            .await?;

        if !res.status().is_success() {
            anyhow::bail!(
                "Claude API error {}: {}",
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
    fn extracts_text_from_claude_response() {
        let body = serde_json::json!({
            "content": [{ "type": "text", "text": "hello" }]
        });
        assert_eq!(extract_text(&body).unwrap(), "hello");
    }

    #[test]
    fn errors_on_unexpected_shape() {
        let body = serde_json::json!({ "content": [] });
        assert!(extract_text(&body).is_err());
    }
}
