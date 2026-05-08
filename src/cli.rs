use clap::{Parser, Subcommand};
use tracing::debug;

use crate::error::{SimError, SimResult};

#[derive(Parser, Debug, Clone)]
#[command(
    name = "ussd-sim",
    version = "0.1.0",
    author = "Your Name",
    about = "Simulate a USSD session against your locally running app",
    long_about = "
ussd-sim lets you test USSD applications locally without a real SIM card or telco connection.
It simulates the full request/response cycle that a telecom gateway would perform.

EXAMPLES:
    ussd-sim dial --code \"*737#\" --app http://localhost:3000/ussd
    ussd-sim replay --file session_20240101_120000_abc12345.json
    ussd-sim config --set-url http://localhost:3000/ussd
    ussd-sim config --show
    "
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,

    /// Enable verbose debug output
    #[arg(short, long, global = true, default_value_t = false)]
    pub verbose: bool,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    /// Dial a USSD code and start an interactive session
    Dial {
        /// The USSD code to dial e.g. *737#
        #[arg(short, long)]
        code: String,

        /// The URL of your USSD app (overrides config)
        #[arg(short, long)]
        app: Option<String>,

        /// Phone number to simulate (overrides config)
        #[arg(short, long)]
        phone: Option<String>,

        /// Maximum number of steps before stopping (overrides config)
        #[arg(short, long)]
        max_steps: Option<u32>,

        /// Request timeout in seconds (overrides config)
        #[arg(short, long)]
        timeout: Option<u64>,

        /// Save session log to file
        #[arg(short, long, default_value_t = true)]
        save_log: bool,
    },

    /// Replay a previously saved session automatically
    Replay {
        /// Path to the session log JSON file
        #[arg(short, long)]
        file: String,

        /// The URL of your USSD app (overrides the one in the log file)
        #[arg(short, long)]
        app: Option<String>,

        /// Delay between steps in milliseconds
        #[arg(short, long, default_value_t = 500)]
        delay_ms: u64,
    },

    /// Manage ussd-sim configuration
    Config {
        /// Show current config
        #[arg(long, default_value_t = false)]
        show: bool,

        /// Set default app URL
        #[arg(long)]
        set_url: Option<String>,

        /// Set default phone number
        #[arg(long)]
        set_phone: Option<String>,

        /// Set max steps
        #[arg(long)]
        set_max_steps: Option<u32>,

        /// Set request timeout in seconds
        #[arg(long)]
        set_timeout: Option<u64>,

        /// Set log directory
        #[arg(long)]
        set_log_dir: Option<String>,
    },
}

impl Cli {
    /// Validates dial arguments
    pub fn validate_dial(
        code: &str,
        app: &Option<String>,
        config_app: &Option<String>,
    ) -> SimResult<String> {
        debug!("Validating dial arguments");

        // Validate USSD code
        if !code.starts_with('*') || !code.ends_with('#') {
            return Err(SimError::InvalidUssdCode(code.to_string()));
        }

        // Resolve app URL: CLI arg > config > error
        let app_url = app
            .clone()
            .or_else(|| config_app.clone())
            .ok_or_else(|| {
                SimError::InvalidAppUrl(
                    "No app URL provided. Use --app or set it with: ussd-sim config --set-url <URL>".to_string()
                )
            })?;

        // Validate app URL
        if !app_url.starts_with("http://") && !app_url.starts_with("https://") {
            return Err(SimError::InvalidAppUrl(app_url));
        }

        debug!("Dial arguments are valid");
        Ok(app_url)
    }
}