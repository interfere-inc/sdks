pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptOperationCursor {
    pub kind: QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptOperationCursorKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptOperationCursor {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptOperationCursorBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptOperationCursorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptOperationCursorBuilder {
    kind: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptOperationCursorKind>,
    cursor: Option<String>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptOperationCursorBuilder {
    pub fn kind(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptOperationCursorKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptOperationCursor`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptOperationCursorBuilder::kind)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptOperationCursor, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindMcpSetupWithdrawalReceiptOperationCursor {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            cursor: self.cursor,
        })
    }
}
