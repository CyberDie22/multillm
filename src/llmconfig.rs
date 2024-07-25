use std::collections::HashMap;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub global: Global,
    pub api: HashMap<String, ApiConfig>,
    pub models: HashMap<String, ModelConfig>,
}

#[derive(Deserialize, Clone)]
pub struct Global {
    pub ip: String,
    pub port: u16,
}

#[derive(Deserialize, Clone)]
pub enum ModelType {
    #[serde(rename = "chat-completion")]
    ChatCompletion
}

#[derive(Deserialize, Clone)]
pub struct ModelConfig {
    #[serde(rename = "api-id")]
    pub api_id: String,
    #[serde(rename = "remote-id")]
    pub remote_id: String,
    #[serde(rename = "model-type")]
    pub model_type: Vec<ModelType>,
    #[serde(rename = "context-length")]
    pub context_length: u32,
    #[serde(rename = "max-input-tokens")]
    pub max_input_tokens: u32,
    #[serde(rename = "max-output-tokens")]
    pub max_output_tokens: u32,
    #[serde(rename = "input-token-cost")]
    pub input_token_cost: f64,
    #[serde(rename = "output-token-cost")]
    pub output_token_cost: f64,
    #[serde(rename = "tokens-per-minute", default)]
    pub tokens_per_minute: Option<u32>,
    #[serde(rename = "requests-per-minute", default)]
    pub requests_per_minute: Option<u32>,
    #[serde(rename = "shares-limits", default)]
    pub shares_limits: Option<bool>,
    #[serde(default)]
    pub group: Option<Vec<String>>,
}

#[derive(Deserialize, Clone)]
pub struct ApiConfig {
    #[serde(rename = "api-type")]
    pub api_type: String,
    #[serde(rename = "api-key-env")]
    pub api_key_env: String,
    // Add other fields that might be common across different API types
}