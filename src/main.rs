mod cli;
mod config;
mod display;
mod error;
mod http;
mod logger;
mod replay;
mod session;

use clap::Parser;
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

use cli::{Cli, Command};
use config::Config;
use display::Display;
use replay::Replay;
use session::{Session, SessionConfig};

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    // Set up tracing
    let filter = if args.verbose {
        "ussd_sim=debug"
    } else {
        "ussd_sim=warn"
    };

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new(filter))
        .with_target(false)
        .without_time()
        .init();

    info!("ussd-sim starting up");

    // Load config from disk
    let config = match Config::load() {
        Ok(c) => c,
        Err(e) => {
            Display::error(&format!("Failed to load config: {}", e));
            std::process::exit(1);
        }
    };

    match args.command {
        // ── DIAL ──────────────────────────────────────────────────
        Command::Dial {
            code,
            app,
            phone,
            max_steps,
            timeout,
            save_log,
        } => {
            // Resolve app URL from CLI or config
            let app_url = match Cli::validate_dial(&code, &app, &config.app_url) {
                Ok(url) => url,
                Err(e) => {
                    Display::error(&e.to_string());
                    std::process::exit(1);
                }
            };

            let phone = phone
                .or_else(|| config.phone.clone())
                .unwrap_or_else(|| "+2348000000000".to_string());

            let session_config = SessionConfig {
                max_steps: max_steps.or(config.max_steps).unwrap_or(20),
                timeout_secs: timeout.or(config.timeout_secs).unwrap_or(30),
                save_log,
                log_dir: config.log_dir.clone(),
            };

            Display::banner(&code, &app_url, &phone);

            let mut session = Session::new(
                code,
                phone,
                app_url,
                session_config,
            );

            if let Err(e) = session.run().await {
                error!("Session failed: {}", e);
                Display::error(&e.to_string());
                std::process::exit(1);
            }
        }

        // ── REPLAY ────────────────────────────────────────────────
        Command::Replay { file, app, delay_ms } => {
            let replay = match Replay::load(&file, app, delay_ms) {
                Ok(r) => r,
                Err(e) => {
                    Display::error(&e.to_string());
                    std::process::exit(1);
                }
            };

            if let Err(e) = replay.run().await {
                error!("Replay failed: {}", e);
                Display::error(&e.to_string());
                std::process::exit(1);
            }
        }

        // ── CONFIG ────────────────────────────────────────────────
        Command::Config {
            show,
            set_url,
            set_phone,
            set_max_steps,
            set_timeout,
            set_log_dir,
        } => {
            let mut updated_config = config.clone();

            // Track if anything changed
            let any_changes = set_url.is_some()
                || set_phone.is_some()
                || set_max_steps.is_some()
                || set_timeout.is_some()
                || set_log_dir.is_some();

            // Apply changes
            if let Some(url) = set_url {
                updated_config.app_url = Some(url);
            }
            if let Some(phone) = set_phone {
                updated_config.phone = Some(phone);
            }
            if let Some(steps) = set_max_steps {
                updated_config.max_steps = Some(steps);
            }
            if let Some(timeout) = set_timeout {
                updated_config.timeout_secs = Some(timeout);
            }
            if let Some(log_dir) = set_log_dir {
                updated_config.log_dir = Some(log_dir);
            }

            // Save if any changes were made
            if any_changes {
                if let Err(e) = updated_config.save() {
                    Display::error(&e.to_string());
                    std::process::exit(1);
                }
                Display::config_saved();
            }

            if show || !any_changes {
                updated_config.print();
            }
        }
    }
}