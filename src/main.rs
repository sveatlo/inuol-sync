mod config;
mod email_notifier;
mod handlers;
mod invoice_ninja_client;
mod models;
mod server;
mod service;
mod uol_client;

use crate::config::Config;
use crate::email_notifier::EmailNotifier;
use crate::invoice_ninja_client::InvoiceNinjaClient;
use crate::server::{create_router, AppState};
use crate::service::SyncService;
use crate::uol_client::UolClient;
use anyhow::Result;
use std::sync::Arc;
use tracing::debug;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load().expect("Failed to load configuration");

    let log_level = config
        .logging
        .level
        .parse::<LevelFilter>()
        .unwrap_or(LevelFilter::INFO);

    tracing_subscriber::registry()
        .with(
            EnvFilter::builder()
                .with_default_directive(log_level.into())
                .from_env_lossy(),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    debug!(?config, "Configuration loaded successfully");
    info!("Starting Invoice Ninja Sync Service");

    let uol_client = UolClient::new(config.uol.clone());
    let invoice_ninja_client = InvoiceNinjaClient::new(config.invoice_ninja.clone());
    let email_notifier =
        EmailNotifier::new(&config.email).expect("Failed to initialize email notifier");

    let sync_service = SyncService::new(uol_client, invoice_ninja_client, email_notifier);

    let state = Arc::new(AppState { sync_service });

    let app = create_router(state);

    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind to address");

    info!(addr, "Server listening");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");

    Ok(())
}
