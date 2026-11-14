pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationKind {
    pub kind: TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationKindKind,
    #[serde(default)]
    pub uri: String,
}

impl TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationKind {
    pub fn builder() -> TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationKindBuilder {
        <TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationKindBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationKindBuilder {
    kind: Option<TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationKindKind>,
    uri: Option<String>,
}

impl TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationKindBuilder {
    pub fn kind(mut self, value: TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationKindKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn uri(mut self, value: impl Into<String>) -> Self {
        self.uri = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationKind`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationKindBuilder::kind)
    /// - [`uri`](TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationKindBuilder::uri)
    pub fn build(self) -> Result<TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationKind, BuildError> {
        Ok(TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperationKind {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            uri: self.uri.ok_or_else(|| BuildError::missing_field("uri"))?,
        })
    }
}
