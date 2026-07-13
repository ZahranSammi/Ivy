use async_trait::async_trait;
use std::time::Duration;

use super::provider::LlmProvider;

const DEFAULT_ENDPOINT: &str = "http://localhost:11434";

pub struct OllamaProvider {
    client: reqwest::Client,
    model: String,
    endpoint: String,
}

impl OllamaProvider {
    pub fn new(model: String, endpoint: Option<String>) -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .expect("reqwest client"),
            model,
            endpoint: endpoint.unwrap_or_else(|| DEFAULT_ENDPOINT.to_string()),
        }
    }
}

fn extract_text(body: &serde_json::Value) -> Result<String, anyhow::Error> {
    body["response"]
        .as_str()
        .map(str::to_string)
        .ok_or_else(|| anyhow::anyhow!("unexpected Ollama response shape: {body}"))
}

#[async_trait]
impl LlmProvider for OllamaProvider {
    async fn generate(&self, prompt: &str) -> Result<String, anyhow::Error> {
        let url = format!("{}/api/generate", self.endpoint.trim_end_matches('/'));
        let res = self
            .client
            .post(&url)
            .json(&serde_json::json!({
                "model": self.model,
                "prompt": prompt,
                "stream": false,
            }))
            .send()
            .await?;

        if !res.status().is_success() {
            anyhow::bail!(
                "Ollama API error {}: {}",
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
    fn extracts_text_from_ollama_response() {
        let body = serde_json::json!({ "response": "hello", "done": true });
        assert_eq!(extract_text(&body).unwrap(), "hello");
    }

    #[test]
    fn errors_on_unexpected_shape() {
        let body = serde_json::json!({ "done": true });
        assert!(extract_text(&body).is_err());
    }
}
