pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceipt {
    #[serde(default)]
    pub at: i64,
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    pub status: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptStatus,
    pub receipt: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceipt,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceipt {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptBuilder {
    at: Option<i64>,
    user_id: Option<String>,
    status: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptStatus>,
    receipt: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceipt>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptBuilder {
    pub fn at(mut self, value: i64) -> Self {
        self.at = Some(value);
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn receipt(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceipt) -> Self {
        self.receipt = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceipt`].
    /// This method will fail if any of the following fields are not set:
    /// - [`at`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptBuilder::at)
    /// - [`status`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptBuilder::status)
    /// - [`receipt`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptBuilder::receipt)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceipt, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceipt {
            at: self.at.ok_or_else(|| BuildError::missing_field("at"))?,
            user_id: self.user_id,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            receipt: self.receipt.ok_or_else(|| BuildError::missing_field("receipt"))?,
        })
    }
}
