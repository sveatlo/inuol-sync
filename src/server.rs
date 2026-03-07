use crate::handlers::{handle_client_created, handle_invoice_created, health_check};
use crate::service::SyncService;
use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;

pub struct AppState {
    pub sync_service: SyncService,
}

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(health_check))
        .route("/health", get(health_check))
        .route("/webhooks/client-created", post(handle_client_created))
        .route("/webhooks/invoice-created", post(handle_invoice_created))
        .with_state(state)
}
