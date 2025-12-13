use super::{AiProvider, ProviderError, ProviderResult};
use async_trait::async_trait;
use reqwest::Client;
use serde_json::{json, Value};

const GEMINI_API_URL: &str = "https://generativelanguage.googleapis.com/v1beta/models";

pub struct GeminiProvider {
    api_key: String,
    model: String,
    client: Client,
}

impl GeminiProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            model: "gemini-flash-lite-latest".to_string(),
            client: Client::new(),
        }
    }
}

#[async_trait]
impl AiProvider for GeminiProvider {
    async fn generate(&self, prompt: &str) -> ProviderResult<String> {
        let url = format!(
            "{}/{}:generateContent",
            GEMINI_API_URL, self.model
        );

        let body = json!({
            "contents": [{
                "parts": [{
                    "text": prompt
                }]
            }],
            "generationConfig": {
                "temperature": 0.7,
                "maxOutputTokens": 500
            }
        });

        let response = self
            .client
            .post(&url)
            .header("x-goog-api-key", &self.api_key)
            .json(&body)
            .send()
            .await
            .map_err(|e| ProviderError::NetworkError(e.to_string()))?;

        let status = response.status().as_u16();

        if status == 429 {
            return Err(ProviderError::RateLimited { retry_after: None });
        }

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(ProviderError::ApiError {
                status,
                message: error_text,
            });
        }

        let json: Value = response
            .json()
            .await
            .map_err(|e| ProviderError::ParseError(e.to_string()))?;

        json["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .map(|s| s.trim().to_string())
            .ok_or_else(|| {
                ProviderError::ParseError("Failed to extract text from response".to_string())
            })
    }

    fn name(&self) -> &'static str {
        "Gemini"
    }

    fn model(&self) -> &str {
        &self.model
    }
}
