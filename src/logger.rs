use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

use chrono::Local;
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

use crate::error::{SimError, SimResult};

/// A single step in a USSD session
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SessionStep {
    pub step: u32,
    pub text_sent: String,
    pub response: String,
    pub user_input: Option<String>,
}

/// Full session log saved to disk
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SessionLog {
    pub session_id: String,
    pub service_code: String,
    pub phone_number: String,
    pub app_url: String,
    pub started_at: String,
    pub ended_at: Option<String>,
    pub completed: bool,
    pub steps: Vec<SessionStep>,
}

impl SessionLog {
    pub fn new(
        session_id: String,
        service_code: String,
        phone_number: String,
        app_url: String,
    ) -> Self {
        Self {
            session_id,
            service_code,
            phone_number,
            app_url,
            started_at: Local::now().to_rfc3339(),
            ended_at: None,
            completed: false,
            steps: Vec::new(),
        }
    }

    pub fn add_step(&mut self, step: SessionStep) {
        debug!("Logging step {}", step.step);
        self.steps.push(step);
    }

    pub fn finish(&mut self, completed: bool) {
        self.ended_at = Some(Local::now().to_rfc3339());
        self.completed = completed;
    }

    /// Saves the session log to a JSON file
    pub fn save(&self, log_dir: Option<&str>) -> SimResult<PathBuf> {
        // Determine log directory
        let dir = if let Some(d) = log_dir {
            PathBuf::from(d)
        } else {
            let base = dirs::data_local_dir()
                .unwrap_or_else(|| PathBuf::from("."));
            base.join("ussd-sim").join("logs")
        };

        // Create directory if needed
        fs::create_dir_all(&dir)
            .map_err(|e| SimError::LogError(e.to_string()))?;

        // File name: session_<timestamp>_<session_id_prefix>.json
        let timestamp = Local::now().format("%Y%m%d_%H%M%S");
        let id_prefix = &self.session_id[..8];
        let filename = format!("session_{}_{}.json", timestamp, id_prefix);
        let path = dir.join(&filename);

        // Write JSON
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| SimError::LogError(e.to_string()))?;

        let mut file = File::create(&path)
            .map_err(|e| SimError::LogError(e.to_string()))?;

        file.write_all(json.as_bytes())
            .map_err(|e| SimError::LogError(e.to_string()))?;

        info!("Session log saved to: {}", path.display());
        Ok(path)
    }
}