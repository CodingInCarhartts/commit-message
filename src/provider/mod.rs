mod openrouter;
mod gemini;

pub use openrouter::OpenRouterProvider;
pub use gemini::GeminiProvider;

use crate::config::{Config, Provider};
use async_trait::async_trait;

pub type ProviderResult<T> = Result<T, ProviderError>;

#[derive(Debug)]
pub enum ProviderError {
    NetworkError(String),
    ApiError { status: u16, message: String },
    ParseError(String),
    RateLimited { retry_after: Option<u64> },
}

impl std::fmt::Display for ProviderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NetworkError(msg) => write!(f, "Network error: {}", msg),
            Self::ApiError { status, message } => write!(f, "API error ({}): {}", status, message),
            Self::ParseError(msg) => write!(f, "Failed to parse response: {}", msg),
            Self::RateLimited { retry_after } => {
                if let Some(secs) = retry_after {
                    write!(f, "Rate limited. Retry after {} seconds", secs)
                } else {
                    write!(f, "Rate limited. Please try again later")
                }
            }
        }
    }
}

impl std::error::Error for ProviderError {}

#[async_trait]
pub trait AiProvider: Send + Sync {
    async fn generate(&self, prompt: &str) -> ProviderResult<String>;
    fn name(&self) -> &'static str;
    fn model(&self) -> &str;
}

pub fn create_provider(config: &Config) -> Box<dyn AiProvider> {
    match config.provider {
        Provider::OpenRouter => Box::new(OpenRouterProvider::new(
            config.openrouter_api_key.clone().unwrap(),
            config.model.clone(),
        )),
        Provider::Gemini => Box::new(GeminiProvider::new(
            config.google_api_key.clone().unwrap(),
        )),
    }
}
