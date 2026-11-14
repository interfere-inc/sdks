pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationDecision {
    #[serde(default)]
    pub attempts: i64,
    pub decision: QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationDecisionDecision,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback: Option<String>,
    pub kind: QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationDecisionKind,
    pub rejection: QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationDecisionRejection,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationDecision {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationDecisionBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationDecisionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationDecisionBuilder {
    attempts: Option<i64>,
    decision: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationDecisionDecision>,
    feedback: Option<String>,
    kind: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationDecisionKind>,
    rejection: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationDecisionRejection>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationDecisionBuilder {
    pub fn attempts(mut self, value: i64) -> Self {
        self.attempts = Some(value);
        self
    }

    pub fn decision(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationDecisionDecision) -> Self {
        self.decision = Some(value);
        self
    }

    pub fn feedback(mut self, value: impl Into<String>) -> Self {
        self.feedback = Some(value.into());
        self
    }

    pub fn kind(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationDecisionKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn rejection(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationDecisionRejection) -> Self {
        self.rejection = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationDecision`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attempts`](QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationDecisionBuilder::attempts)
    /// - [`decision`](QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationDecisionBuilder::decision)
    /// - [`kind`](QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationDecisionBuilder::kind)
    /// - [`rejection`](QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationDecisionBuilder::rejection)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationDecision, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationDecision {
            attempts: self.attempts.ok_or_else(|| BuildError::missing_field("attempts"))?,
            decision: self.decision.ok_or_else(|| BuildError::missing_field("decision"))?,
            feedback: self.feedback,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            rejection: self.rejection.ok_or_else(|| BuildError::missing_field("rejection"))?,
        })
    }
}
