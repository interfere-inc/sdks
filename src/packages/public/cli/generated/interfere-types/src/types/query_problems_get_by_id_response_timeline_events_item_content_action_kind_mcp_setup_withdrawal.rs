pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawal {
    pub receipt: QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalReceipt,
    #[serde(default)]
    pub at: i64,
    #[serde(default)]
    pub reason: String,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawal {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalBuilder {
    receipt: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalReceipt>,
    at: Option<i64>,
    reason: Option<String>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalBuilder {
    pub fn receipt(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalReceipt) -> Self {
        self.receipt = Some(value);
        self
    }

    pub fn at(mut self, value: i64) -> Self {
        self.at = Some(value);
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawal`].
    /// This method will fail if any of the following fields are not set:
    /// - [`receipt`](QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalBuilder::receipt)
    /// - [`at`](QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalBuilder::at)
    /// - [`reason`](QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalBuilder::reason)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawal, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawal {
            receipt: self.receipt.ok_or_else(|| BuildError::missing_field("receipt"))?,
            at: self.at.ok_or_else(|| BuildError::missing_field("at"))?,
            reason: self.reason.ok_or_else(|| BuildError::missing_field("reason"))?,
        })
    }
}
