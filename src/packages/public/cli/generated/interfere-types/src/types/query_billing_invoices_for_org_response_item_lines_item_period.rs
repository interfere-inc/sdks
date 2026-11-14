pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryBillingInvoicesForOrgResponseItemLinesItemPeriod {
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub start: f64,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub end: f64,
}

impl QueryBillingInvoicesForOrgResponseItemLinesItemPeriod {
    pub fn builder() -> QueryBillingInvoicesForOrgResponseItemLinesItemPeriodBuilder {
        <QueryBillingInvoicesForOrgResponseItemLinesItemPeriodBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryBillingInvoicesForOrgResponseItemLinesItemPeriodBuilder {
    start: Option<f64>,
    end: Option<f64>,
}

impl QueryBillingInvoicesForOrgResponseItemLinesItemPeriodBuilder {
    pub fn start(mut self, value: f64) -> Self {
        self.start = Some(value);
        self
    }

    pub fn end(mut self, value: f64) -> Self {
        self.end = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryBillingInvoicesForOrgResponseItemLinesItemPeriod`].
    /// This method will fail if any of the following fields are not set:
    /// - [`start`](QueryBillingInvoicesForOrgResponseItemLinesItemPeriodBuilder::start)
    /// - [`end`](QueryBillingInvoicesForOrgResponseItemLinesItemPeriodBuilder::end)
    pub fn build(self) -> Result<QueryBillingInvoicesForOrgResponseItemLinesItemPeriod, BuildError> {
        Ok(QueryBillingInvoicesForOrgResponseItemLinesItemPeriod {
            start: self.start.ok_or_else(|| BuildError::missing_field("start"))?,
            end: self.end.ok_or_else(|| BuildError::missing_field("end"))?,
        })
    }
}
