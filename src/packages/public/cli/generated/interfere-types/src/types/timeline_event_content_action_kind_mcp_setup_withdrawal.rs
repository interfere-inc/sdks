pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentActionKindMcpSetupWithdrawal {
    pub receipt: TimelineEventContentActionKindMcpSetupWithdrawalReceipt,
    #[serde(default)]
    pub at: i64,
    #[serde(default)]
    pub reason: String,
}

impl TimelineEventContentActionKindMcpSetupWithdrawal {
    pub fn builder() -> TimelineEventContentActionKindMcpSetupWithdrawalBuilder {
        <TimelineEventContentActionKindMcpSetupWithdrawalBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentActionKindMcpSetupWithdrawalBuilder {
    receipt: Option<TimelineEventContentActionKindMcpSetupWithdrawalReceipt>,
    at: Option<i64>,
    reason: Option<String>,
}

impl TimelineEventContentActionKindMcpSetupWithdrawalBuilder {
    pub fn receipt(mut self, value: TimelineEventContentActionKindMcpSetupWithdrawalReceipt) -> Self {
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

    /// Consumes the builder and constructs a [`TimelineEventContentActionKindMcpSetupWithdrawal`].
    /// This method will fail if any of the following fields are not set:
    /// - [`receipt`](TimelineEventContentActionKindMcpSetupWithdrawalBuilder::receipt)
    /// - [`at`](TimelineEventContentActionKindMcpSetupWithdrawalBuilder::at)
    /// - [`reason`](TimelineEventContentActionKindMcpSetupWithdrawalBuilder::reason)
    pub fn build(self) -> Result<TimelineEventContentActionKindMcpSetupWithdrawal, BuildError> {
        Ok(TimelineEventContentActionKindMcpSetupWithdrawal {
            receipt: self.receipt.ok_or_else(|| BuildError::missing_field("receipt"))?,
            at: self.at.ok_or_else(|| BuildError::missing_field("at"))?,
            reason: self.reason.ok_or_else(|| BuildError::missing_field("reason"))?,
        })
    }
}
