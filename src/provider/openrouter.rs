use super::{AiProvider, ProviderError, ProviderResult};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

const OPENROUTER_API_URL: &str = "https://openrouter.ai/api/v1/chat/completions";

pub struct OpenRouterProvider {
    api_key: String,
    model: String,
    client: Client,
}

impl OpenRouterProvider {
    pub fn new(api_key: String, model: String) -> Self {
        Self {
            api_key,
            model,
            client: Client::new(),
        }
    }
}

#[derive(Serialize)]
struct OpenRouterRequest {
    model: String,
    messages: Vec<Message>,
    max_tokens: u32,
    temperature: f32,
}

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct OpenRouterResponse {
    choices: Option<Vec<Choice>>,
    error: Option<ApiErrorResponse>,
}

#[derive(Deserialize)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Deserialize)]
struct ResponseMessage {
    content: String,
}

#[derive(Deserialize)]
struct ApiErrorResponse {
    message: String,
}

#[async_trait]
impl AiProvider for OpenRouterProvider {
    async fn generate(&self, prompt: &str) -> ProviderResult<String> {
        let request = OpenRouterRequest {
            model: self.model.clone(),
            messages: vec![Message {
                role: "user".to_string(),
                content: prompt.to_string(),
            }],
            max_tokens: 500,
            temperature: 0.7,
        };

        let response = self
            .client
            .post(OPENROUTER_API_URL)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header(
                "HTTP-Referer",
                "https://github.com/CodingInCarhartts/commit-message",
            )
            .header("X-Title", "Commit Message Generator")
            .json(&request)
            .send()
            .await
            .map_err(|e| ProviderError::NetworkError(e.to_string()))?;

        let status = response.status().as_u16();

        if status == 429 {
            let retry_after = response
                .headers()
                .get("retry-after")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse().ok());
            return Err(ProviderError::RateLimited { retry_after });
        }

        let body: OpenRouterResponse = response
            .json()
            .await
            .map_err(|e| ProviderError::ParseError(e.to_string()))?;

        if let Some(error) = body.error {
            return Err(ProviderError::ApiError {
                status,
                message: error.message,
            });
        }

        body.choices
            .and_then(|c| c.into_iter().next())
            .map(|choice| choice.message.content.trim().to_string())
            .ok_or_else(|| ProviderError::ParseError("No choices in response".to_string()))
    }

    fn name(&self) -> &'static str {
        "OpenRouter"
    }

    fn model(&self) -> &str {
        &self.model
    }
}
