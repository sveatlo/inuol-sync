use crate::models::{ClientWebhook, InvoiceWebhook};
use crate::server::AppState;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use std::sync::Arc;
use tracing::{error, info};

pub async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "inuol-sync"
    }))
}

pub async fn handle_client_created(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ClientWebhook>,
) -> impl IntoResponse {
    info!(client_name = %payload.name, "Received client created webhook");

    match state.sync_service.sync_client(payload).await {
        Ok(()) => (
            StatusCode::OK,
            Json(serde_json::json!({"status": "success", "message": "Client synced successfully"})),
        ),
        Err(e) => {
            error!(error = %e, "Failed to process client webhook");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(
                    serde_json::json!({"status": "error", "message": format!("Failed to sync client: {}", e)}),
                ),
            )
        }
    }
}

pub async fn handle_invoice_created(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<InvoiceWebhook>,
) -> impl IntoResponse {
    info!(invoice_number = %payload.number, "Received invoice created webhook");

    match state.sync_service.sync_invoice(payload).await {
        Ok(()) => (
            StatusCode::OK,
            Json(
                serde_json::json!({"status": "success", "message": "Invoice synced successfully"}),
            ),
        ),
        Err(e) => {
            error!(error = %e, "Failed to process invoice webhook");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(
                    serde_json::json!({"status": "error", "message": format!("Failed to sync invoice: {}", e)}),
                ),
            )
        }
    }
}
