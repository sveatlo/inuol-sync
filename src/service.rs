use crate::email_notifier::EmailNotifier;
use crate::invoice_ninja_client::InvoiceNinjaClient;
use crate::models::{ClientWebhook, InvoiceWebhook, UolContact, UolInvoice};
use crate::uol_client::UolClient;
use anyhow::{Context, Result};
use tracing::info;

pub struct SyncService {
    uol_client: UolClient,
    invoice_ninja_client: InvoiceNinjaClient,
    email_notifier: EmailNotifier,
}

impl SyncService {
    pub fn new(
        uol_client: UolClient,
        invoice_ninja_client: InvoiceNinjaClient,
        email_notifier: EmailNotifier,
    ) -> Self {
        Self {
            uol_client,
            invoice_ninja_client,
            email_notifier,
        }
    }

    pub async fn sync_client(&self, client: ClientWebhook) -> Result<()> {
        info!(client_name = %client.name, "Processing client created webhook");
        if let Err(e) = self.do_sync_client(&client).await {
            self.email_notifier.notify_error(&e).await;
            return Err(e);
        }
        info!(client_name = %client.name, "Successfully synced client to UOL");
        Ok(())
    }

    async fn do_sync_client(&self, client: &ClientWebhook) -> Result<()> {
        let uol_contact = UolContact::try_from(client).context("transforming client")?;
        let contact_id = self
            .uol_client
            .create_contact(uol_contact)
            .await
            .context("creating UOL contact")?;
        self.invoice_ninja_client
            .update_client_uol_reference(&client.id, &contact_id)
            .await
            .context("updating Invoice Ninja client")?;
        Ok(())
    }

    pub async fn sync_invoice(&self, invoice: InvoiceWebhook) -> Result<()> {
        info!(invoice_number = %invoice.number, "Processing invoice created webhook");
        if let Err(e) = self.do_sync_invoice(&invoice).await {
            self.email_notifier.notify_error(&e).await;
            return Err(e);
        }
        info!(invoice_number = %invoice.number, "Successfully synced invoice to UOL");
        Ok(())
    }

    async fn do_sync_invoice(&self, invoice: &InvoiceWebhook) -> Result<()> {
        let uol_invoice = UolInvoice::try_from(invoice).context("transforming invoice")?;
        let uol_invoice_id = self
            .uol_client
            .create_invoice(uol_invoice)
            .await
            .context("creating UOL invoice")?;

        if invoice.number != uol_invoice_id {
            info!(invoice_number = %invoice.number, %uol_invoice_id, "Invoice number differs, updating");
            self.invoice_ninja_client
                .update_invoice_number(&invoice.id, &uol_invoice_id)
                .await
                .context("updating Invoice Ninja invoice number")?;
        }

        Ok(())
    }
}
