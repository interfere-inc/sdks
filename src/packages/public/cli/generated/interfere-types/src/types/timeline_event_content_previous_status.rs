pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TimelineEventContentPreviousStatus {
    #[serde(rename = "recoveryEvidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_evidence: Option<TimelineEventContentPreviousStatusRecoveryEvidence>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry: Option<TimelineEventContentPreviousStatusRetry>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TimelineEventContentPreviousStatusStatus>,
    #[serde(rename = "previousStatus")]
    pub previous_status: TimelineEventContentPreviousStatusPreviousStatus,
}

impl TimelineEventContentPreviousStatus {
    pub fn builder() -> TimelineEventContentPreviousStatusBuilder {
        <TimelineEventContentPreviousStatusBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentPreviousStatusBuilder {
    recovery_evidence: Option<TimelineEventContentPreviousStatusRecoveryEvidence>,
    retry: Option<TimelineEventContentPreviousStatusRetry>,
    status: Option<TimelineEventContentPreviousStatusStatus>,
    previous_status: Option<TimelineEventContentPreviousStatusPreviousStatus>,
}

impl TimelineEventContentPreviousStatusBuilder {
    pub fn recovery_evidence(mut self, value: TimelineEventContentPreviousStatusRecoveryEvidence) -> Self {
        self.recovery_evidence = Some(value);
        self
    }

    pub fn retry(mut self, value: TimelineEventContentPreviousStatusRetry) -> Self {
        self.retry = Some(value);
        self
    }

    pub fn status(mut self, value: TimelineEventContentPreviousStatusStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn previous_status(mut self, value: TimelineEventContentPreviousStatusPreviousStatus) -> Self {
        self.previous_status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentPreviousStatus`].
    /// This method will fail if any of the following fields are not set:
    /// - [`previous_status`](TimelineEventContentPreviousStatusBuilder::previous_status)
    pub fn build(self) -> Result<TimelineEventContentPreviousStatus, BuildError> {
        Ok(TimelineEventContentPreviousStatus {
            recovery_evidence: self.recovery_evidence,
            retry: self.retry,
            status: self.status,
            previous_status: self.previous_status.ok_or_else(|| BuildError::missing_field("previous_status"))?,
        })
    }
}
