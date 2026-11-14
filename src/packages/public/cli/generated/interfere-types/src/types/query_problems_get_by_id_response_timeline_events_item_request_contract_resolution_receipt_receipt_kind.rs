pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKind {
    pub kind: QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKindKind,
    #[serde(rename = "observedAt")]
    #[serde(default)]
    pub observed_at: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence: Option<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKindEvidence>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKind {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKindBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKindBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKindBuilder {
    kind: Option<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKindKind>,
    observed_at: Option<i64>,
    evidence: Option<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKindEvidence>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKindBuilder {
    pub fn kind(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKindKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn observed_at(mut self, value: i64) -> Self {
        self.observed_at = Some(value);
        self
    }

    pub fn evidence(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKindEvidence) -> Self {
        self.evidence = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKind`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKindBuilder::kind)
    /// - [`observed_at`](QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKindBuilder::observed_at)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKind, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKind {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            observed_at: self.observed_at.ok_or_else(|| BuildError::missing_field("observed_at"))?,
            evidence: self.evidence,
        })
    }
}
