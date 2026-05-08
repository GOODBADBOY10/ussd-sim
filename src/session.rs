use std::io::{self, Write};
use std::time::Duration;
use tokio::time::timeout;
use tracing::{debug, info, instrument, warn};
use uuid::Uuid;

use crate::{
    display::Display,
    error::{SimResult, SimError},
    http::{UssdAction, UssdHttpClient, UssdRequest},
    logger::{SessionLog, SessionStep},
};

pub struct SessionConfig {
    pub max_steps: u32,
    pub timeout_secs: u64,
    pub save_log: bool,
    pub log_dir: Option<String>,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            max_steps: 20,
            timeout_secs: 30,
            save_log: true,
            log_dir: None,
        }
    }
}

pub struct Session {
    session_id: String,
    service_code: String,
    phone_number: String,
    text: String,
    step: u32,
    client: UssdHttpClient,
    config: SessionConfig,
    log: SessionLog,
}

impl Session {
    pub fn new(
        service_code: String,
        phone_number: String,
        app_url: String,
        config: SessionConfig,
    ) -> Self {
        let session_id = Uuid::new_v4().to_string();
        info!("Creating new session with ID: {}", session_id);

        let log = SessionLog::new(
            session_id.clone(),
            service_code.clone(),
            phone_number.clone(),
            app_url.clone(),
        );

        Self {
            session_id,
            service_code,
            phone_number,
            text: String::new(),
            step: 0,
            client: UssdHttpClient::new(app_url, config.timeout_secs),
            config,
            log,
        }
    }

    #[instrument(skip(self), fields(session_id = %self.session_id))]
    pub async fn run(&mut self) -> SimResult<()> {
        info!("Starting USSD session");

        let result = self.run_inner().await;

        // Mark log as completed or failed
        self.log.finish(result.is_ok());

        // Save log if enabled
        if self.config.save_log {
            match self.log.save(self.config.log_dir.as_deref()) {
                Ok(path) => Display::log_saved(&path.display().to_string()),
                Err(e) => warn!("Failed to save session log: {}", e),
            }
        }

        result
    }

    async fn run_inner(&mut self) -> SimResult<()> {
        loop {
            self.step += 1;
            debug!("Session step {}, text: \"{}\"", self.step, self.text);

            // Max steps protection
            if self.step > self.config.max_steps {
                warn!("Max steps ({}) reached", self.config.max_steps);
                return Err(SimError::MaxStepsReached(self.config.max_steps));
            }

            let request = UssdRequest {
                session_id: self.session_id.clone(),
                service_code: self.service_code.clone(),
                phone_number: self.phone_number.clone(),
                text: self.text.clone(),
            };

            // Send request with timeout
            let response = timeout(
                Duration::from_secs(self.config.timeout_secs),
                self.client.send(&request),
            )
            .await
            .map_err(|_| SimError::Timeout(self.config.timeout_secs))??;

            match response.action {
                UssdAction::Continue => {
                    info!("Step {}: received CON response", self.step);
                    Display::menu(&response.message);
                    Display::step(self.step, &self.text);

                    let input = Self::read_input()?;

                    // Log this step
                    self.log.add_step(SessionStep {
                        step: self.step,
                        text_sent: self.text.clone(),
                        response: format!("CON {}", response.message),
                        user_input: Some(input.clone()),
                    });

                    if self.text.is_empty() {
                        self.text = input;
                    } else {
                        self.text = format!("{}*{}", self.text, input);
                    }

                    debug!("Updated text: \"{}\"", self.text);
                }

                UssdAction::End => {
                    info!("Session ended at step {}", self.step);

                    // Log final step
                    self.log.add_step(SessionStep {
                        step: self.step,
                        text_sent: self.text.clone(),
                        response: format!("END {}", response.message),
                        user_input: None,
                    });

                    Display::end(&response.message);
                    return Ok(());
                }
            }
        }
    }

    fn read_input() -> SimResult<String> {
        print!("Enter input: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        Ok(input.trim().to_string())
    }
}