use crate::config::InvoiceNinjaConfig;
use crate::models::{UpdateClientUolRefRequest, UpdateInvoiceRequest};
use reqwest::{Client, StatusCode};
use tracing::{info, warn};

#[derive(Debug, thiserror::Error)]
pub enum InvoiceNinjaError {
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),

    #[error("API returned error {status}: {body}")]
    Api { status: StatusCode, body: String },
}

pub struct InvoiceNinjaClient {
    client: Client,
    config: InvoiceNinjaConfig,
}

impl InvoiceNinjaClient {
    pub fn new(config: InvoiceNinjaConfig) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Failed to build HTTP client");

        Self { client, config }
    }

    pub async fn update_client_uol_reference(
        &self,
        client_id: &str,
        contact_id: &str,
    ) -> Result<(), InvoiceNinjaError> {
        let url = format!("{}/clients/{}", self.config.base_url, client_id);

        let body = UpdateClientUolRefRequest {
            custom_value1: contact_id.to_owned(),
        };

        info!(client_id, contact_id, "Updating client");

        let response = self
            .client
            .put(&url)
            .header("X-API-TOKEN", &self.config.api_token)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        if response.status().is_success() {
            info!(client_id, "Successfully updated client");
            Ok(())
        } else {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            warn!(%status, %body, "Failed to update client");
            Err(InvoiceNinjaError::Api { status, body })
        }
    }

    pub async fn update_invoice_number(
        &self,
        invoice_id: &str,
        new_number: &str,
    ) -> Result<(), InvoiceNinjaError> {
        let url = format!("{}/invoices/{}", self.config.base_url, invoice_id);

        let body = UpdateInvoiceRequest {
            number: new_number.to_owned(),
        };

        info!(invoice_id, new_number, "Updating invoice");

        let response = self
            .client
            .put(&url)
            .header("X-API-TOKEN", &self.config.api_token)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        if response.status().is_success() {
            info!(invoice_id, "Successfully updated invoice");
            Ok(())
        } else {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            warn!(%status, %body, "Failed to update invoice");
            Err(InvoiceNinjaError::Api { status, body })
        }
    }
}
