pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TimelineEventContentEvidenceGapPublicationReason {
    #[serde(default)]
    pub attempts: i64,
    pub kind: TimelineEventContentEvidenceGapPublicationReasonKind,
    pub reason: TimelineEventContentEvidenceGapPublicationReasonReason,
    pub rejection: TimelineEventContentEvidenceGapPublicationReasonRejection,
}

impl TimelineEventContentEvidenceGapPublicationReason {
    pub fn builder() -> TimelineEventContentEvidenceGapPublicationReasonBuilder {
        <TimelineEventContentEvidenceGapPublicationReasonBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentEvidenceGapPublicationReasonBuilder {
    attempts: Option<i64>,
    kind: Option<TimelineEventContentEvidenceGapPublicationReasonKind>,
    reason: Option<TimelineEventContentEvidenceGapPublicationReasonReason>,
    rejection: Option<TimelineEventContentEvidenceGapPublicationReasonRejection>,
}

impl TimelineEventContentEvidenceGapPublicationReasonBuilder {
    pub fn attempts(mut self, value: i64) -> Self {
        self.attempts = Some(value);
        self
    }

    pub fn kind(mut self, value: TimelineEventContentEvidenceGapPublicationReasonKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn reason(mut self, value: TimelineEventContentEvidenceGapPublicationReasonReason) -> Self {
        self.reason = Some(value);
        self
    }

    pub fn rejection(mut self, value: TimelineEventContentEvidenceGapPublicationReasonRejection) -> Self {
        self.rejection = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentEvidenceGapPublicationReason`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attempts`](TimelineEventContentEvidenceGapPublicationReasonBuilder::attempts)
    /// - [`kind`](TimelineEventContentEvidenceGapPublicationReasonBuilder::kind)
    /// - [`reason`](TimelineEventContentEvidenceGapPublicationReasonBuilder::reason)
    /// - [`rejection`](TimelineEventContentEvidenceGapPublicationReasonBuilder::rejection)
    pub fn build(self) -> Result<TimelineEventContentEvidenceGapPublicationReason, BuildError> {
        Ok(TimelineEventContentEvidenceGapPublicationReason {
            attempts: self.attempts.ok_or_else(|| BuildError::missing_field("attempts"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            reason: self.reason.ok_or_else(|| BuildError::missing_field("reason"))?,
            rejection: self.rejection.ok_or_else(|| BuildError::missing_field("rejection"))?,
        })
    }
}
