use thiserror::Error;

#[derive(Debug, Error)]
pub enum SimError {
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("Failed to read user input: {0}")]
    InputError(#[from] std::io::Error),

    #[error("Invalid response from app: expected 'CON' or 'END' prefix, got: '{0}'")]
    InvalidResponse(String),

    #[error("App is unreachable at '{url}': {reason}")]
    AppUnreachable { url: String, reason: String },

    #[error("Invalid USSD code '{0}': must start with * and end with #")]
    InvalidUssdCode(String),

    #[error("Invalid app URL '{0}': must be a valid HTTP/HTTPS URL")]
    InvalidAppUrl(String),

    #[error("Session reached maximum steps ({0}) — possible infinite loop detected")]
    MaxStepsReached(u32),

    #[error("Request timed out after {0} seconds")]
    Timeout(u64),

    #[error("Failed to write session log: {0}")]
    LogError(String),

    #[error("Failed to read config file: {0}")]
    ConfigError(String),

    #[error("Replay file not found: '{0}'")]
    ReplayFileNotFound(String),

    #[error("Invalid replay file: {0}")]
    InvalidReplayFile(String),

    #[allow(dead_code)]
    #[error("Session ended unexpectedly")]
    SessionTerminated,
}

pub type SimResult<T> = Result<T, SimError>;