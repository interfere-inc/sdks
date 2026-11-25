pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationBasis {
    #[serde(default)]
    pub attempts: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basis: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationBasisBasis>,
    pub decision: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationBasisDecision,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback: Option<String>,
    pub kind: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationBasisKind,
    pub rejection: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationBasisRejection,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationBasis {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationBasisBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationBasisBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationBasisBuilder {
    attempts: Option<i64>,
    basis: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationBasisBasis>,
    decision: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationBasisDecision>,
    feedback: Option<String>,
    kind: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationBasisKind>,
    rejection: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationBasisRejection>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationBasisBuilder {
    pub fn attempts(mut self, value: i64) -> Self {
        self.attempts = Some(value);
        self
    }

    pub fn basis(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationBasisBasis) -> Self {
        self.basis = Some(value);
        self
    }

    pub fn decision(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationBasisDecision) -> Self {
        self.decision = Some(value);
        self
    }

    pub fn feedback(mut self, value: impl Into<String>) -> Self {
        self.feedback = Some(value.into());
        self
    }

    pub fn kind(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationBasisKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn rejection(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationBasisRejection) -> Self {
        self.rejection = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationBasis`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attempts`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationBasisBuilder::attempts)
    /// - [`decision`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationBasisBuilder::decision)
    /// - [`kind`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationBasisBuilder::kind)
    /// - [`rejection`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationBasisBuilder::rejection)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationBasis, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationBasis {
            attempts: self.attempts.ok_or_else(|| BuildError::missing_field("attempts"))?,
            basis: self.basis,
            decision: self.decision.ok_or_else(|| BuildError::missing_field("decision"))?,
            feedback: self.feedback,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            rejection: self.rejection.ok_or_else(|| BuildError::missing_field("rejection"))?,
        })
    }
}
