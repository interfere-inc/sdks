pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceipt {
    pub kind: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptKind,
    #[serde(rename = "connectionRevision")]
    #[serde(default)]
    pub connection_revision: i64,
    pub operation: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptOperation,
    pub provenance: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenance,
    #[serde(rename = "contentDigest")]
    #[serde(default)]
    pub content_digest: String,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceipt {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptBuilder {
    kind: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptKind>,
    connection_revision: Option<i64>,
    operation: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptOperation>,
    provenance: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenance>,
    content_digest: Option<String>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptBuilder {
    pub fn kind(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn connection_revision(mut self, value: i64) -> Self {
        self.connection_revision = Some(value);
        self
    }

    pub fn operation(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptOperation) -> Self {
        self.operation = Some(value);
        self
    }

    pub fn provenance(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptProvenance) -> Self {
        self.provenance = Some(value);
        self
    }

    pub fn content_digest(mut self, value: impl Into<String>) -> Self {
        self.content_digest = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceipt`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptBuilder::kind)
    /// - [`connection_revision`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptBuilder::connection_revision)
    /// - [`operation`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptBuilder::operation)
    /// - [`provenance`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptBuilder::provenance)
    /// - [`content_digest`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptBuilder::content_digest)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceipt, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindMcpSetupWithdrawalReceipt {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            connection_revision: self.connection_revision.ok_or_else(|| BuildError::missing_field("connection_revision"))?,
            operation: self.operation.ok_or_else(|| BuildError::missing_field("operation"))?,
            provenance: self.provenance.ok_or_else(|| BuildError::missing_field("provenance"))?,
            content_digest: self.content_digest.ok_or_else(|| BuildError::missing_field("content_digest"))?,
        })
    }
}
