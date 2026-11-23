pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TimelineEventContentActionKindEvidenceStatus {
    #[serde(rename = "capabilityReady")]
    #[serde(default)]
    pub capability_ready: bool,
    #[serde(rename = "requestedEvidenceArrived")]
    #[serde(default)]
    pub requested_evidence_arrived: bool,
    /// Milliseconds since the Unix epoch in UTC.
    #[serde(rename = "checkedAt")]
    #[serde(default)]
    pub checked_at: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt: Option<TimelineEventContentActionKindEvidenceStatusReceipt>,
}

impl TimelineEventContentActionKindEvidenceStatus {
    pub fn builder() -> TimelineEventContentActionKindEvidenceStatusBuilder {
        <TimelineEventContentActionKindEvidenceStatusBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentActionKindEvidenceStatusBuilder {
    capability_ready: Option<bool>,
    requested_evidence_arrived: Option<bool>,
    checked_at: Option<i64>,
    failure: Option<String>,
    receipt: Option<TimelineEventContentActionKindEvidenceStatusReceipt>,
}

impl TimelineEventContentActionKindEvidenceStatusBuilder {
    pub fn capability_ready(mut self, value: bool) -> Self {
        self.capability_ready = Some(value);
        self
    }

    pub fn requested_evidence_arrived(mut self, value: bool) -> Self {
        self.requested_evidence_arrived = Some(value);
        self
    }

    pub fn checked_at(mut self, value: i64) -> Self {
        self.checked_at = Some(value);
        self
    }

    pub fn failure(mut self, value: impl Into<String>) -> Self {
        self.failure = Some(value.into());
        self
    }

    pub fn receipt(mut self, value: TimelineEventContentActionKindEvidenceStatusReceipt) -> Self {
        self.receipt = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentActionKindEvidenceStatus`].
    /// This method will fail if any of the following fields are not set:
    /// - [`capability_ready`](TimelineEventContentActionKindEvidenceStatusBuilder::capability_ready)
    /// - [`requested_evidence_arrived`](TimelineEventContentActionKindEvidenceStatusBuilder::requested_evidence_arrived)
    /// - [`checked_at`](TimelineEventContentActionKindEvidenceStatusBuilder::checked_at)
    pub fn build(self) -> Result<TimelineEventContentActionKindEvidenceStatus, BuildError> {
        Ok(TimelineEventContentActionKindEvidenceStatus {
            capability_ready: self.capability_ready.ok_or_else(|| BuildError::missing_field("capability_ready"))?,
            requested_evidence_arrived: self.requested_evidence_arrived.ok_or_else(|| BuildError::missing_field("requested_evidence_arrived"))?,
            checked_at: self.checked_at.ok_or_else(|| BuildError::missing_field("checked_at"))?,
            failure: self.failure,
            receipt: self.receipt,
        })
    }
}
