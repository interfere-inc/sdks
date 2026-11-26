pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentDismissal {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dismissal: Option<TimelineEventContentDismissalDismissal>,
    #[serde(rename = "recoveryEvidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_evidence: Option<TimelineEventContentDismissalRecoveryEvidence>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<TimelineEventContentDismissalRecurrence>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry: Option<TimelineEventContentDismissalRetry>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TimelineEventContentDismissalStatus>,
    #[serde(rename = "previousStatus")]
    pub previous_status: TimelineEventContentDismissalPreviousStatus,
}

impl TimelineEventContentDismissal {
    pub fn builder() -> TimelineEventContentDismissalBuilder {
        <TimelineEventContentDismissalBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentDismissalBuilder {
    dismissal: Option<TimelineEventContentDismissalDismissal>,
    recovery_evidence: Option<TimelineEventContentDismissalRecoveryEvidence>,
    recurrence: Option<TimelineEventContentDismissalRecurrence>,
    retry: Option<TimelineEventContentDismissalRetry>,
    status: Option<TimelineEventContentDismissalStatus>,
    previous_status: Option<TimelineEventContentDismissalPreviousStatus>,
}

impl TimelineEventContentDismissalBuilder {
    pub fn dismissal(mut self, value: TimelineEventContentDismissalDismissal) -> Self {
        self.dismissal = Some(value);
        self
    }

    pub fn recovery_evidence(mut self, value: TimelineEventContentDismissalRecoveryEvidence) -> Self {
        self.recovery_evidence = Some(value);
        self
    }

    pub fn recurrence(mut self, value: TimelineEventContentDismissalRecurrence) -> Self {
        self.recurrence = Some(value);
        self
    }

    pub fn retry(mut self, value: TimelineEventContentDismissalRetry) -> Self {
        self.retry = Some(value);
        self
    }

    pub fn status(mut self, value: TimelineEventContentDismissalStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn previous_status(mut self, value: TimelineEventContentDismissalPreviousStatus) -> Self {
        self.previous_status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentDismissal`].
    /// This method will fail if any of the following fields are not set:
    /// - [`previous_status`](TimelineEventContentDismissalBuilder::previous_status)
    pub fn build(self) -> Result<TimelineEventContentDismissal, BuildError> {
        Ok(TimelineEventContentDismissal {
            dismissal: self.dismissal,
            recovery_evidence: self.recovery_evidence,
            recurrence: self.recurrence,
            retry: self.retry,
            status: self.status,
            previous_status: self.previous_status.ok_or_else(|| BuildError::missing_field("previous_status"))?,
        })
    }
}
