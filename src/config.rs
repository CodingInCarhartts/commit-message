use std::env;

/// Supported AI providers
#[derive(Debug, Clone, PartialEq)]
pub enum Provider {
    OpenRouter,
    Gemini,
}

impl Default for Provider {
    fn default() -> Self {
        Provider::OpenRouter
    }
}

/// Application configuration
#[derive(Debug, Clone)]
pub struct Config {
    pub provider: Provider,
    pub model: String,
    pub emoji_enabled: bool,
    pub max_diff_lines: usize,
    pub min_message_length: usize,
    pub max_retries: u32,
    pub openrouter_api_key: Option<String>,
    pub google_api_key: Option<String>,
}

#[derive(Debug)]
pub enum ConfigError {
    MissingApiKey(&'static str),
    InvalidProvider(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingApiKey(key) => write!(f, "Missing required environment variable: {}", key),
            Self::InvalidProvider(p) => write!(f, "Invalid provider '{}'. Use 'openrouter' or 'gemini'", p),
        }
    }
}

impl std::error::Error for ConfigError {}

impl Config {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self, ConfigError> {
        let provider = match env::var("CM_PROVIDER").ok().as_deref() {
            Some("gemini") => Provider::Gemini,
            Some("openrouter") | None => Provider::OpenRouter,
            Some(other) => return Err(ConfigError::InvalidProvider(other.to_string())),
        };

        let model = env::var("CM_MODEL").unwrap_or_else(|_| {
            match provider {
                Provider::OpenRouter => "kwaipilot/kat-coder-pro:free".to_string(),
                Provider::Gemini => "gemini-flash-lite-latest".to_string(),
            }
        });

        let emoji_enabled = env::var("CM_EMOJI")
            .map(|v| v != "0" && v.to_lowercase() != "false")
            .unwrap_or(true);

        let max_diff_lines = env::var("CM_MAX_DIFF_LINES")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(200);

        let min_message_length = env::var("CM_MIN_LENGTH")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(20);

        let openrouter_api_key = env::var("OPENROUTER_API_KEY").ok();
        let google_api_key = env::var("GOOGLE_API_KEY").ok();

        match provider {
            Provider::OpenRouter if openrouter_api_key.is_none() => {
                return Err(ConfigError::MissingApiKey("OPENROUTER_API_KEY"));
            }
            Provider::Gemini if google_api_key.is_none() => {
                return Err(ConfigError::MissingApiKey("GOOGLE_API_KEY"));
            }
            _ => {}
        }

        Ok(Self {
            provider,
            model,
            emoji_enabled,
            max_diff_lines,
            min_message_length,
            max_retries: 3,
            openrouter_api_key,
            google_api_key,
        })
    }

    /// Get the API key for the current provider
    pub fn api_key(&self) -> &str {
        match self.provider {
            Provider::OpenRouter => self.openrouter_api_key.as_ref().unwrap(),
            Provider::Gemini => self.google_api_key.as_ref().unwrap(),
        }
    }
}
