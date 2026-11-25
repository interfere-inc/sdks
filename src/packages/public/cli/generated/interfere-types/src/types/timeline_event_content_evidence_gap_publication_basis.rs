pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TimelineEventContentEvidenceGapPublicationBasis {
    #[serde(default)]
    pub attempts: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basis: Option<TimelineEventContentEvidenceGapPublicationBasisBasis>,
    pub decision: TimelineEventContentEvidenceGapPublicationBasisDecision,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback: Option<String>,
    pub kind: TimelineEventContentEvidenceGapPublicationBasisKind,
    pub rejection: TimelineEventContentEvidenceGapPublicationBasisRejection,
}

impl TimelineEventContentEvidenceGapPublicationBasis {
    pub fn builder() -> TimelineEventContentEvidenceGapPublicationBasisBuilder {
        <TimelineEventContentEvidenceGapPublicationBasisBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentEvidenceGapPublicationBasisBuilder {
    attempts: Option<i64>,
    basis: Option<TimelineEventContentEvidenceGapPublicationBasisBasis>,
    decision: Option<TimelineEventContentEvidenceGapPublicationBasisDecision>,
    feedback: Option<String>,
    kind: Option<TimelineEventContentEvidenceGapPublicationBasisKind>,
    rejection: Option<TimelineEventContentEvidenceGapPublicationBasisRejection>,
}

impl TimelineEventContentEvidenceGapPublicationBasisBuilder {
    pub fn attempts(mut self, value: i64) -> Self {
        self.attempts = Some(value);
        self
    }

    pub fn basis(mut self, value: TimelineEventContentEvidenceGapPublicationBasisBasis) -> Self {
        self.basis = Some(value);
        self
    }

    pub fn decision(mut self, value: TimelineEventContentEvidenceGapPublicationBasisDecision) -> Self {
        self.decision = Some(value);
        self
    }

    pub fn feedback(mut self, value: impl Into<String>) -> Self {
        self.feedback = Some(value.into());
        self
    }

    pub fn kind(mut self, value: TimelineEventContentEvidenceGapPublicationBasisKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn rejection(mut self, value: TimelineEventContentEvidenceGapPublicationBasisRejection) -> Self {
        self.rejection = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentEvidenceGapPublicationBasis`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attempts`](TimelineEventContentEvidenceGapPublicationBasisBuilder::attempts)
    /// - [`decision`](TimelineEventContentEvidenceGapPublicationBasisBuilder::decision)
    /// - [`kind`](TimelineEventContentEvidenceGapPublicationBasisBuilder::kind)
    /// - [`rejection`](TimelineEventContentEvidenceGapPublicationBasisBuilder::rejection)
    pub fn build(self) -> Result<TimelineEventContentEvidenceGapPublicationBasis, BuildError> {
        Ok(TimelineEventContentEvidenceGapPublicationBasis {
            attempts: self.attempts.ok_or_else(|| BuildError::missing_field("attempts"))?,
            basis: self.basis,
            decision: self.decision.ok_or_else(|| BuildError::missing_field("decision"))?,
            feedback: self.feedback,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            rejection: self.rejection.ok_or_else(|| BuildError::missing_field("rejection"))?,
        })
    }
}
