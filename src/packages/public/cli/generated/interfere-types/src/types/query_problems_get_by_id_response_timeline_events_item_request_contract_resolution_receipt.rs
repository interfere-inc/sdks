pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceipt {
    #[serde(default)]
    pub at: i64,
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    pub status: QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptStatus,
    pub receipt: QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceipt,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceipt {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptBuilder {
    at: Option<i64>,
    user_id: Option<String>,
    status: Option<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptStatus>,
    receipt: Option<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceipt>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptBuilder {
    pub fn at(mut self, value: i64) -> Self {
        self.at = Some(value);
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn receipt(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceipt) -> Self {
        self.receipt = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceipt`].
    /// This method will fail if any of the following fields are not set:
    /// - [`at`](QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptBuilder::at)
    /// - [`status`](QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptBuilder::status)
    /// - [`receipt`](QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptBuilder::receipt)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceipt, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceipt {
            at: self.at.ok_or_else(|| BuildError::missing_field("at"))?,
            user_id: self.user_id,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            receipt: self.receipt.ok_or_else(|| BuildError::missing_field("receipt"))?,
        })
    }
}
