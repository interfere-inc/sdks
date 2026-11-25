pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationBasis {
    #[serde(default)]
    pub attempts: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basis: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationBasisBasis>,
    pub decision: QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationBasisDecision,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback: Option<String>,
    pub kind: QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationBasisKind,
    pub rejection: QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationBasisRejection,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationBasis {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationBasisBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationBasisBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationBasisBuilder {
    attempts: Option<i64>,
    basis: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationBasisBasis>,
    decision: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationBasisDecision>,
    feedback: Option<String>,
    kind: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationBasisKind>,
    rejection: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationBasisRejection>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationBasisBuilder {
    pub fn attempts(mut self, value: i64) -> Self {
        self.attempts = Some(value);
        self
    }

    pub fn basis(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationBasisBasis) -> Self {
        self.basis = Some(value);
        self
    }

    pub fn decision(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationBasisDecision) -> Self {
        self.decision = Some(value);
        self
    }

    pub fn feedback(mut self, value: impl Into<String>) -> Self {
        self.feedback = Some(value.into());
        self
    }

    pub fn kind(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationBasisKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn rejection(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationBasisRejection) -> Self {
        self.rejection = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationBasis`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attempts`](QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationBasisBuilder::attempts)
    /// - [`decision`](QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationBasisBuilder::decision)
    /// - [`kind`](QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationBasisBuilder::kind)
    /// - [`rejection`](QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationBasisBuilder::rejection)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationBasis, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationBasis {
            attempts: self.attempts.ok_or_else(|| BuildError::missing_field("attempts"))?,
            basis: self.basis,
            decision: self.decision.ok_or_else(|| BuildError::missing_field("decision"))?,
            feedback: self.feedback,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            rejection: self.rejection.ok_or_else(|| BuildError::missing_field("rejection"))?,
        })
    }
}
