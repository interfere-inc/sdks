pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryBillingInvoicesForOrgResponseItemLinesItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<QueryBillingInvoicesForOrgResponseItemLinesItemPeriod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<f64>,
}

impl QueryBillingInvoicesForOrgResponseItemLinesItem {
    pub fn builder() -> QueryBillingInvoicesForOrgResponseItemLinesItemBuilder {
        <QueryBillingInvoicesForOrgResponseItemLinesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryBillingInvoicesForOrgResponseItemLinesItemBuilder {
    amount: Option<f64>,
    description: Option<String>,
    period: Option<QueryBillingInvoicesForOrgResponseItemLinesItemPeriod>,
    quantity: Option<f64>,
}

impl QueryBillingInvoicesForOrgResponseItemLinesItemBuilder {
    pub fn amount(mut self, value: f64) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn period(mut self, value: QueryBillingInvoicesForOrgResponseItemLinesItemPeriod) -> Self {
        self.period = Some(value);
        self
    }

    pub fn quantity(mut self, value: f64) -> Self {
        self.quantity = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryBillingInvoicesForOrgResponseItemLinesItem`].
    pub fn build(self) -> Result<QueryBillingInvoicesForOrgResponseItemLinesItem, BuildError> {
        Ok(QueryBillingInvoicesForOrgResponseItemLinesItem {
            amount: self.amount,
            description: self.description,
            period: self.period,
            quantity: self.quantity,
        })
    }
}
