pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryBillingInvoicesForOrgResponseItem {
    #[serde(rename = "amountPaid")]
    #[serde(default)]
    pub amount_paid: i64,
    #[serde(default)]
    pub currency: String,
    #[serde(rename = "hostedInvoiceUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_invoice_url: Option<String>,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "invoicePdf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_pdf: Option<String>,
    #[serde(default)]
    pub lines: Vec<QueryBillingInvoicesForOrgResponseItemLinesItem>,
    #[serde(rename = "organizationId")]
    #[serde(default)]
    pub organization_id: String,
    #[serde(rename = "periodEnd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub period_end: Option<f64>,
    #[serde(rename = "periodStart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub period_start: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<QueryBillingInvoicesForOrgResponseItemStatus>,
    #[serde(rename = "stripeInvoiceId")]
    #[serde(default)]
    pub stripe_invoice_id: String,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub created_at: f64,
    #[serde(rename = "deletedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub deleted_at: Option<f64>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub updated_at: f64,
}

impl QueryBillingInvoicesForOrgResponseItem {
    pub fn builder() -> QueryBillingInvoicesForOrgResponseItemBuilder {
        <QueryBillingInvoicesForOrgResponseItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryBillingInvoicesForOrgResponseItemBuilder {
    amount_paid: Option<i64>,
    currency: Option<String>,
    hosted_invoice_url: Option<String>,
    id: Option<String>,
    invoice_pdf: Option<String>,
    lines: Option<Vec<QueryBillingInvoicesForOrgResponseItemLinesItem>>,
    organization_id: Option<String>,
    period_end: Option<f64>,
    period_start: Option<f64>,
    status: Option<QueryBillingInvoicesForOrgResponseItemStatus>,
    stripe_invoice_id: Option<String>,
    created_at: Option<f64>,
    deleted_at: Option<f64>,
    updated_at: Option<f64>,
}

impl QueryBillingInvoicesForOrgResponseItemBuilder {
    pub fn amount_paid(mut self, value: i64) -> Self {
        self.amount_paid = Some(value);
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn hosted_invoice_url(mut self, value: impl Into<String>) -> Self {
        self.hosted_invoice_url = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn invoice_pdf(mut self, value: impl Into<String>) -> Self {
        self.invoice_pdf = Some(value.into());
        self
    }

    pub fn lines(mut self, value: Vec<QueryBillingInvoicesForOrgResponseItemLinesItem>) -> Self {
        self.lines = Some(value);
        self
    }

    pub fn organization_id(mut self, value: impl Into<String>) -> Self {
        self.organization_id = Some(value.into());
        self
    }

    pub fn period_end(mut self, value: f64) -> Self {
        self.period_end = Some(value);
        self
    }

    pub fn period_start(mut self, value: f64) -> Self {
        self.period_start = Some(value);
        self
    }

    pub fn status(mut self, value: QueryBillingInvoicesForOrgResponseItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn stripe_invoice_id(mut self, value: impl Into<String>) -> Self {
        self.stripe_invoice_id = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: f64) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn deleted_at(mut self, value: f64) -> Self {
        self.deleted_at = Some(value);
        self
    }

    pub fn updated_at(mut self, value: f64) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryBillingInvoicesForOrgResponseItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount_paid`](QueryBillingInvoicesForOrgResponseItemBuilder::amount_paid)
    /// - [`currency`](QueryBillingInvoicesForOrgResponseItemBuilder::currency)
    /// - [`id`](QueryBillingInvoicesForOrgResponseItemBuilder::id)
    /// - [`lines`](QueryBillingInvoicesForOrgResponseItemBuilder::lines)
    /// - [`organization_id`](QueryBillingInvoicesForOrgResponseItemBuilder::organization_id)
    /// - [`stripe_invoice_id`](QueryBillingInvoicesForOrgResponseItemBuilder::stripe_invoice_id)
    /// - [`created_at`](QueryBillingInvoicesForOrgResponseItemBuilder::created_at)
    /// - [`updated_at`](QueryBillingInvoicesForOrgResponseItemBuilder::updated_at)
    pub fn build(self) -> Result<QueryBillingInvoicesForOrgResponseItem, BuildError> {
        Ok(QueryBillingInvoicesForOrgResponseItem {
            amount_paid: self.amount_paid.ok_or_else(|| BuildError::missing_field("amount_paid"))?,
            currency: self.currency.ok_or_else(|| BuildError::missing_field("currency"))?,
            hosted_invoice_url: self.hosted_invoice_url,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            invoice_pdf: self.invoice_pdf,
            lines: self.lines.ok_or_else(|| BuildError::missing_field("lines"))?,
            organization_id: self.organization_id.ok_or_else(|| BuildError::missing_field("organization_id"))?,
            period_end: self.period_end,
            period_start: self.period_start,
            status: self.status,
            stripe_invoice_id: self.stripe_invoice_id.ok_or_else(|| BuildError::missing_field("stripe_invoice_id"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            deleted_at: self.deleted_at,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
