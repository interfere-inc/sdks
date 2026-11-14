pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationCursor {
    pub kind: TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationCursorKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationCursor {
    pub fn builder() -> TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationCursorBuilder {
        <TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationCursorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationCursorBuilder {
    kind: Option<TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationCursorKind>,
    cursor: Option<String>,
}

impl TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationCursorBuilder {
    pub fn kind(mut self, value: TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationCursorKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationCursor`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationCursorBuilder::kind)
    pub fn build(self) -> Result<TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationCursor, BuildError> {
        Ok(TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationCursor {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            cursor: self.cursor,
        })
    }
}
