use crate::config::UolConfig;
use crate::models::{UolContact, UolContactResponse, UolInvoice, UolInvoiceResponse};
use reqwest::{Client, StatusCode};
use tracing::{info, warn};

#[derive(Debug, thiserror::Error)]
pub enum UolError {
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),

    #[error("API returned error {status}: {body}")]
    Api { status: StatusCode, body: String },
}

pub struct UolClient {
    client: Client,
    config: UolConfig,
}

impl UolClient {
    pub fn new(config: UolConfig) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Failed to build HTTP client");

        Self { client, config }
    }

    pub async fn create_contact(&self, contact: UolContact) -> Result<String, UolError> {
        let url = format!("{}/contacts", self.config.base_url);

        info!(contact_name = %contact.name, "Creating contact in UOL");

        let response = self
            .client
            .post(&url)
            .basic_auth(&self.config.username, Some(&self.config.password))
            .header("Content-Type", "application/json")
            .json(&contact)
            .send()
            .await?;

        if response.status().is_success() {
            let contact_response: UolContactResponse = response.json().await?;
            info!(contact_id = %contact_response.contact_id, "Successfully created contact");
            Ok(contact_response.contact_id)
        } else {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            warn!(%status, %body, "Failed to create contact");
            Err(UolError::Api { status, body })
        }
    }

    pub async fn create_invoice(&self, invoice: UolInvoice) -> Result<String, UolError> {
        let url = format!("{}/sales_invoices", self.config.base_url);

        info!(buyer_id = %invoice.buyer_id, "Creating invoice in UOL");

        let response = self
            .client
            .post(&url)
            .basic_auth(&self.config.username, Some(&self.config.password))
            .header("Content-Type", "application/json")
            .json(&invoice)
            .send()
            .await?;

        if response.status().is_success() {
            let invoice_response: UolInvoiceResponse = response.json().await?;
            info!(invoice_id = invoice_response.invoice_id, public_id = %invoice_response.public_id, "Successfully created invoice");
            Ok(invoice_response.public_id)
        } else {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            warn!(%status, %body, "Failed to create invoice");
            Err(UolError::Api { status, body })
        }
    }
}
