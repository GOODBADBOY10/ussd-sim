use reqwest::Client;
use serde::Serialize;
use tracing::{debug, instrument};

use crate::error::{SimError, SimResult};

#[derive(Serialize, Debug, Clone)]
pub struct UssdRequest {
    #[serde(rename = "sessionId")]
    pub session_id: String,

    #[serde(rename = "serviceCode")]
    pub service_code: String,

    #[serde(rename = "phoneNumber")]
    pub phone_number: String,

    pub text: String,
}

#[derive(Debug, Clone)]
pub struct UssdResponse {
    pub action: UssdAction,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UssdAction {
    Continue,
    End,
}

impl UssdResponse {
    pub fn parse(raw: &str) -> SimResult<Self> {
        if let Some(msg) = raw.strip_prefix("CON ") {
            Ok(UssdResponse {
                action: UssdAction::Continue,
                message: msg.trim().to_string(),
            })
        } else if let Some(msg) = raw.strip_prefix("END ") {
            Ok(UssdResponse {
                action: UssdAction::End,
                message: msg.trim().to_string(),
            })
        } else {
            Err(SimError::InvalidResponse(raw.to_string()))
        }
    }
}

pub struct UssdHttpClient {
    client: Client,
    app_url: String,
}

impl UssdHttpClient {
    pub fn new(app_url: String, timeout_secs: u64) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(timeout_secs))
            .build()
            .expect("Failed to build HTTP client");

        Self { client, app_url }
    }

    #[instrument(skip(self), fields(url = %self.app_url, text = %request.text))]
    pub async fn send(&self, request: &UssdRequest) -> SimResult<UssdResponse> {
        debug!("Sending USSD request to app");
        debug!("Request body: {:?}", request);

        let response = self
            .client
            .post(&self.app_url)
            .json(request)
            .send()
            .await
            .map_err(|e| SimError::AppUnreachable {
                url: self.app_url.clone(),
                reason: e.to_string(),
            })?;

        let status = response.status();
        debug!("App responded with status: {}", status);

        let body = response
            .text()
            .await
            .map_err(|e| SimError::AppUnreachable {
                url: self.app_url.clone(),
                reason: e.to_string(),
            })?;

        debug!("Raw response body: {}", body);

        UssdResponse::parse(&body)
    }
}