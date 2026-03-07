use anyhow::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientWebhook {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub address1: Option<String>,
    pub address2: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country_id: String,
    pub phone: Option<String>,
    pub vat_number: Option<String>,
    pub id_number: Option<String>,
    pub settings: ClientSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientSettings {
    pub currency_id: Option<String>,
    pub language_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceWebhook {
    pub id: String,
    pub user_id: String,
    pub client_id: String,
    pub number: String,
    pub line_items: Vec<LineItem>,
    pub client: ClientInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineItem {
    pub quantity: f64,
    pub cost: f64,
    pub product_key: String,
    pub custom_value1: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientInfo {
    pub custom_value1: Option<String>,
    pub settings: ClientInfoSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientInfoSettings {
    pub currency_id: Option<String>,
}

// UOL Request Models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UolContact {
    pub name: String,
    pub vatin: Option<String>,
    pub vat_payer: bool,
    pub company_number: Option<String>,
    pub country_id: String,
    pub language_id: String,
    pub addresses: Vec<UolAddress>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UolAddress {
    pub name: String,
    pub country_id: String,
    pub phone: String,
    pub street: String,
    pub city: String,
    pub postal_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UolContactResponse {
    pub contact_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UolInvoice {
    pub buyer_id: String,
    pub currency_id: String,
    pub items: Vec<UolInvoiceItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UolInvoiceItem {
    pub product_id: String,
    pub unit_price_vat_inclusive: f64,
    pub quantity: f64,
    pub vat_calculation_method: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UolInvoiceResponse {
    pub invoice_id: u64,
    pub public_id: String,
}

impl TryFrom<&ClientWebhook> for UolContact {
    type Error = anyhow::Error;

    fn try_from(client: &ClientWebhook) -> Result<Self> {
        let country_id = match client.country_id.as_str() {
            "840" => "US",
            "703" => "SK",
            "203" => "CZ",
            _ => bail!("unknown country_id: {:?}", client.country_id),
        }
        .to_owned();

        let language_id = match client.settings.language_id.as_deref() {
            Some("1") => "en-US",
            _ => "cs-CZ",
        }
        .to_owned();

        let vat_payer = client
            .vat_number
            .as_ref()
            .is_some_and(|v| !v.trim().is_empty());

        let address = UolAddress {
            name: client.name.clone(),
            country_id: country_id.clone(),
            phone: client.phone.clone().unwrap_or_default(),
            street: client.address1.clone().unwrap_or_default(),
            city: client.city.clone().unwrap_or_default(),
            postal_code: client.postal_code.clone().unwrap_or_default(),
        };

        Ok(UolContact {
            name: client.name.clone(),
            vatin: client.vat_number.clone(),
            vat_payer,
            company_number: client.id_number.clone(),
            country_id,
            language_id,
            addresses: vec![address],
        })
    }
}

impl TryFrom<&InvoiceWebhook> for UolInvoice {
    type Error = anyhow::Error;

    fn try_from(invoice: &InvoiceWebhook) -> Result<Self> {
        let currency_id = match invoice.client.settings.currency_id.as_deref() {
            Some("3") => "EUR",
            Some("51") => "CZK",
            _ => bail!(
                "unknown currency_id: {:?}",
                invoice.client.settings.currency_id
            ),
        }
        .to_owned();

        let items: Vec<UolInvoiceItem> = invoice
            .line_items
            .iter()
            .map(|item| UolInvoiceItem {
                product_id: item.custom_value1.clone().unwrap_or_default(),
                unit_price_vat_inclusive: item.cost,
                quantity: item.quantity,
                vat_calculation_method: "from_above".to_owned(),
            })
            .collect();

        Ok(UolInvoice {
            buyer_id: invoice
                .client
                .custom_value1
                .as_ref()
                .ok_or_else(|| anyhow!("missing client uol ID"))?
                .clone(),
            currency_id,
            items,
        })
    }
}

// Invoice Ninja Update Models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateClientUolRefRequest {
    pub custom_value1: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateInvoiceRequest {
    pub number: String,
}
