pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptConnection {
    pub kind: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionKind,
    pub connection: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionConnection,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionEvidence>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptConnection {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionBuilder {
    kind: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionKind>,
    connection: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionConnection>,
    evidence: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionEvidence>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionBuilder {
    pub fn kind(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn connection(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionConnection) -> Self {
        self.connection = Some(value);
        self
    }

    pub fn evidence(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionEvidence) -> Self {
        self.evidence = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptConnection`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionBuilder::kind)
    /// - [`connection`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionBuilder::connection)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptConnection, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionReceiptReceiptConnection {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            connection: self.connection.ok_or_else(|| BuildError::missing_field("connection"))?,
            evidence: self.evidence,
        })
    }
}
