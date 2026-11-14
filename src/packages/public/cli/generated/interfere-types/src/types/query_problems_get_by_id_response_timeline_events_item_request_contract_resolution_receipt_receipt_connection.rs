pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnection {
    pub kind: QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionKind,
    pub connection: QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionConnection,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence: Option<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionEvidence>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnection {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionBuilder {
    kind: Option<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionKind>,
    connection: Option<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionConnection>,
    evidence: Option<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionEvidence>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionBuilder {
    pub fn kind(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn connection(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionConnection) -> Self {
        self.connection = Some(value);
        self
    }

    pub fn evidence(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionEvidence) -> Self {
        self.evidence = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnection`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionBuilder::kind)
    /// - [`connection`](QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionBuilder::connection)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnection, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnection {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            connection: self.connection.ok_or_else(|| BuildError::missing_field("connection"))?,
            evidence: self.evidence,
        })
    }
}
