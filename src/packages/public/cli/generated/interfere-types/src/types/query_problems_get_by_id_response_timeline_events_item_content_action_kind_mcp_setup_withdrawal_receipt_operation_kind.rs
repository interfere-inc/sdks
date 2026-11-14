pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptOperationKind {
    pub kind: QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptOperationKindKind,
    #[serde(default)]
    pub uri: String,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptOperationKind {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptOperationKindBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptOperationKindBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptOperationKindBuilder {
    kind: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptOperationKindKind>,
    uri: Option<String>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptOperationKindBuilder {
    pub fn kind(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptOperationKindKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn uri(mut self, value: impl Into<String>) -> Self {
        self.uri = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptOperationKind`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptOperationKindBuilder::kind)
    /// - [`uri`](QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptOperationKindBuilder::uri)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptOperationKind, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptOperationKind {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            uri: self.uri.ok_or_else(|| BuildError::missing_field("uri"))?,
        })
    }
}
