pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationDecision {
    #[serde(default)]
    pub attempts: i64,
    pub decision: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationDecisionDecision,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback: Option<String>,
    pub kind: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationDecisionKind,
    pub rejection: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationDecisionRejection,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationDecision {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationDecisionBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationDecisionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationDecisionBuilder {
    attempts: Option<i64>,
    decision: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationDecisionDecision>,
    feedback: Option<String>,
    kind: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationDecisionKind>,
    rejection: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationDecisionRejection>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationDecisionBuilder {
    pub fn attempts(mut self, value: i64) -> Self {
        self.attempts = Some(value);
        self
    }

    pub fn decision(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationDecisionDecision) -> Self {
        self.decision = Some(value);
        self
    }

    pub fn feedback(mut self, value: impl Into<String>) -> Self {
        self.feedback = Some(value.into());
        self
    }

    pub fn kind(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationDecisionKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn rejection(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationDecisionRejection) -> Self {
        self.rejection = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationDecision`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attempts`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationDecisionBuilder::attempts)
    /// - [`decision`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationDecisionBuilder::decision)
    /// - [`kind`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationDecisionBuilder::kind)
    /// - [`rejection`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationDecisionBuilder::rejection)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationDecision, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationDecision {
            attempts: self.attempts.ok_or_else(|| BuildError::missing_field("attempts"))?,
            decision: self.decision.ok_or_else(|| BuildError::missing_field("decision"))?,
            feedback: self.feedback,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            rejection: self.rejection.ok_or_else(|| BuildError::missing_field("rejection"))?,
        })
    }
}
