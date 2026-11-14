pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TimelineEventContentEvidenceGapPublicationDecision {
    #[serde(default)]
    pub attempts: i64,
    pub decision: TimelineEventContentEvidenceGapPublicationDecisionDecision,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback: Option<String>,
    pub kind: TimelineEventContentEvidenceGapPublicationDecisionKind,
    pub rejection: TimelineEventContentEvidenceGapPublicationDecisionRejection,
}

impl TimelineEventContentEvidenceGapPublicationDecision {
    pub fn builder() -> TimelineEventContentEvidenceGapPublicationDecisionBuilder {
        <TimelineEventContentEvidenceGapPublicationDecisionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentEvidenceGapPublicationDecisionBuilder {
    attempts: Option<i64>,
    decision: Option<TimelineEventContentEvidenceGapPublicationDecisionDecision>,
    feedback: Option<String>,
    kind: Option<TimelineEventContentEvidenceGapPublicationDecisionKind>,
    rejection: Option<TimelineEventContentEvidenceGapPublicationDecisionRejection>,
}

impl TimelineEventContentEvidenceGapPublicationDecisionBuilder {
    pub fn attempts(mut self, value: i64) -> Self {
        self.attempts = Some(value);
        self
    }

    pub fn decision(mut self, value: TimelineEventContentEvidenceGapPublicationDecisionDecision) -> Self {
        self.decision = Some(value);
        self
    }

    pub fn feedback(mut self, value: impl Into<String>) -> Self {
        self.feedback = Some(value.into());
        self
    }

    pub fn kind(mut self, value: TimelineEventContentEvidenceGapPublicationDecisionKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn rejection(mut self, value: TimelineEventContentEvidenceGapPublicationDecisionRejection) -> Self {
        self.rejection = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentEvidenceGapPublicationDecision`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attempts`](TimelineEventContentEvidenceGapPublicationDecisionBuilder::attempts)
    /// - [`decision`](TimelineEventContentEvidenceGapPublicationDecisionBuilder::decision)
    /// - [`kind`](TimelineEventContentEvidenceGapPublicationDecisionBuilder::kind)
    /// - [`rejection`](TimelineEventContentEvidenceGapPublicationDecisionBuilder::rejection)
    pub fn build(self) -> Result<TimelineEventContentEvidenceGapPublicationDecision, BuildError> {
        Ok(TimelineEventContentEvidenceGapPublicationDecision {
            attempts: self.attempts.ok_or_else(|| BuildError::missing_field("attempts"))?,
            decision: self.decision.ok_or_else(|| BuildError::missing_field("decision"))?,
            feedback: self.feedback,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            rejection: self.rejection.ok_or_else(|| BuildError::missing_field("rejection"))?,
        })
    }
}
