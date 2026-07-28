export type LlmProvider = "claude" | "gemini" | "ollama";

export interface LlmConfig {
  provider: LlmProvider;
  model_name: string | null;
  endpoint_url: string | null;
}

export interface LlmConfigUpdate {
  provider: LlmProvider;
  model_name?: string;
  // Write-only: the backend must never echo this back in a GET response.
  api_key: string;
  endpoint_url?: string;
}
