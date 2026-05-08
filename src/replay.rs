use std::time::Duration;
use tokio::time::sleep;
use tracing::{debug, info, instrument};

use crate::{
    display::Display,
    error::{SimError, SimResult},
    http::{UssdHttpClient, UssdRequest},
    logger::SessionLog,
};

pub struct Replay {
    log: SessionLog,
    app_url: String,
    delay_ms: u64,
}

impl Replay {
    pub fn load(file: &str, app_url_override: Option<String>, delay_ms: u64) -> SimResult<Self> {
        // Check file exists
        if !std::path::Path::new(file).exists() {
            return Err(SimError::ReplayFileNotFound(file.to_string()));
        }

        // Read and parse the log file
        let contents = std::fs::read_to_string(file)
            .map_err(|e| SimError::InvalidReplayFile(e.to_string()))?;

        let log: SessionLog = serde_json::from_str(&contents)
            .map_err(|e| SimError::InvalidReplayFile(e.to_string()))?;

        // Use override URL or the one from the log file
        let app_url = app_url_override.unwrap_or_else(|| log.app_url.clone());

        info!("Loaded replay from: {}", file);
        debug!("Replay has {} steps", log.steps.len());

        Ok(Self { log, app_url, delay_ms })
    }

    #[instrument(skip(self))]
    pub async fn run(&self) -> SimResult<()> {
        Display::replay_banner(
            &self.log.service_code,
            &self.app_url,
            &self.log.phone_number,
            self.log.steps.len(),
        );

        let client = UssdHttpClient::new(self.app_url.clone(), 30);

        for step in &self.log.steps {
            info!("Replaying step {}", step.step);

            // Delay between steps
            if step.step > 1 {
                sleep(Duration::from_millis(self.delay_ms)).await;
            }

            let request = UssdRequest {
                session_id: self.log.session_id.clone(),
                service_code: self.log.service_code.clone(),
                phone_number: self.log.phone_number.clone(),
                text: step.text_sent.clone(),
            };

            let response = client.send(&request).await?;

            // Show the response
            Display::replay_step(
                step.step,
                &step.text_sent,
                step.user_input.as_deref(),
                &response.message,
            );

            debug!("Step {} response: {:?}", step.step, response.action);
        }

        Display::replay_done();
        Ok(())
    }
}