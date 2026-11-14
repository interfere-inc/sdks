pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentActionKindMcpSetupWithdrawalReceipt {
    pub kind: TimelineEventContentActionKindMcpSetupWithdrawalReceiptKind,
    #[serde(rename = "connectionRevision")]
    #[serde(default)]
    pub connection_revision: i64,
    pub operation: TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperation,
    pub provenance: TimelineEventContentActionKindMcpSetupWithdrawalReceiptProvenance,
    #[serde(rename = "contentDigest")]
    #[serde(default)]
    pub content_digest: String,
}

impl TimelineEventContentActionKindMcpSetupWithdrawalReceipt {
    pub fn builder() -> TimelineEventContentActionKindMcpSetupWithdrawalReceiptBuilder {
        <TimelineEventContentActionKindMcpSetupWithdrawalReceiptBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentActionKindMcpSetupWithdrawalReceiptBuilder {
    kind: Option<TimelineEventContentActionKindMcpSetupWithdrawalReceiptKind>,
    connection_revision: Option<i64>,
    operation: Option<TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperation>,
    provenance: Option<TimelineEventContentActionKindMcpSetupWithdrawalReceiptProvenance>,
    content_digest: Option<String>,
}

impl TimelineEventContentActionKindMcpSetupWithdrawalReceiptBuilder {
    pub fn kind(mut self, value: TimelineEventContentActionKindMcpSetupWithdrawalReceiptKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn connection_revision(mut self, value: i64) -> Self {
        self.connection_revision = Some(value);
        self
    }

    pub fn operation(mut self, value: TimelineEventContentActionKindMcpSetupWithdrawalReceiptOperation) -> Self {
        self.operation = Some(value);
        self
    }

    pub fn provenance(mut self, value: TimelineEventContentActionKindMcpSetupWithdrawalReceiptProvenance) -> Self {
        self.provenance = Some(value);
        self
    }

    pub fn content_digest(mut self, value: impl Into<String>) -> Self {
        self.content_digest = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentActionKindMcpSetupWithdrawalReceipt`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](TimelineEventContentActionKindMcpSetupWithdrawalReceiptBuilder::kind)
    /// - [`connection_revision`](TimelineEventContentActionKindMcpSetupWithdrawalReceiptBuilder::connection_revision)
    /// - [`operation`](TimelineEventContentActionKindMcpSetupWithdrawalReceiptBuilder::operation)
    /// - [`provenance`](TimelineEventContentActionKindMcpSetupWithdrawalReceiptBuilder::provenance)
    /// - [`content_digest`](TimelineEventContentActionKindMcpSetupWithdrawalReceiptBuilder::content_digest)
    pub fn build(self) -> Result<TimelineEventContentActionKindMcpSetupWithdrawalReceipt, BuildError> {
        Ok(TimelineEventContentActionKindMcpSetupWithdrawalReceipt {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            connection_revision: self.connection_revision.ok_or_else(|| BuildError::missing_field("connection_revision"))?,
            operation: self.operation.ok_or_else(|| BuildError::missing_field("operation"))?,
            provenance: self.provenance.ok_or_else(|| BuildError::missing_field("provenance"))?,
            content_digest: self.content_digest.ok_or_else(|| BuildError::missing_field("content_digest"))?,
        })
    }
}
