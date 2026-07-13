use serde::{Deserialize, Serialize};

use super::claude::ClaudeProvider;
use super::gemini::GeminiProvider;
use super::ollama::OllamaProvider;
use super::openai::OpenAiProvider;
use super::provider::LlmProvider;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LlmProviderKind {
    Gemini,
    Claude,
    #[serde(rename = "openai")]
    OpenAi,
    Ollama,
}

#[derive(Debug, Clone)]
pub struct LlmProviderConfig {
    pub kind: LlmProviderKind,
    pub api_key: String,
    pub model: String,
    pub endpoint: Option<String>,
}

/// Builds the concrete LLM client for a chosen, configured provider — the
/// "LLM connection object" that is FR-007's output once a user picks and
/// configures a provider.
pub fn build_provider(config: LlmProviderConfig) -> Box<dyn LlmProvider> {
    match config.kind {
        LlmProviderKind::Gemini => Box::new(GeminiProvider::new(
            config.api_key,
            config.model,
            config.endpoint,
        )),
        LlmProviderKind::Claude => Box::new(ClaudeProvider::new(
            config.api_key,
            config.model,
            config.endpoint,
        )),
        LlmProviderKind::OpenAi => Box::new(OpenAiProvider::new(
            config.api_key,
            config.model,
            config.endpoint,
        )),
        LlmProviderKind::Ollama => Box::new(OllamaProvider::new(config.model, config.endpoint)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provider_kind_serializes_snake_case() {
        assert_eq!(
            serde_json::to_string(&LlmProviderKind::Gemini).unwrap(),
            "\"gemini\""
        );
        assert_eq!(
            serde_json::to_string(&LlmProviderKind::OpenAi).unwrap(),
            "\"openai\""
        );
    }

    #[test]
    fn provider_kind_roundtrips() {
        let decoded: LlmProviderKind = serde_json::from_str("\"ollama\"").unwrap();
        assert_eq!(decoded, LlmProviderKind::Ollama);
    }
}
