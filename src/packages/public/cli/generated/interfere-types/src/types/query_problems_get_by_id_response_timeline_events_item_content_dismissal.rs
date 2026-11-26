pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentDismissal {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dismissal: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalDismissal>,
    #[serde(rename = "recoveryEvidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_evidence: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalRecoveryEvidence>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalRecurrence>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalRetry>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalStatus>,
    #[serde(rename = "previousStatus")]
    pub previous_status: QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalPreviousStatus,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentDismissal {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalBuilder {
    dismissal: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalDismissal>,
    recovery_evidence: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalRecoveryEvidence>,
    recurrence: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalRecurrence>,
    retry: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalRetry>,
    status: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalStatus>,
    previous_status: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalPreviousStatus>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalBuilder {
    pub fn dismissal(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalDismissal) -> Self {
        self.dismissal = Some(value);
        self
    }

    pub fn recovery_evidence(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalRecoveryEvidence) -> Self {
        self.recovery_evidence = Some(value);
        self
    }

    pub fn recurrence(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalRecurrence) -> Self {
        self.recurrence = Some(value);
        self
    }

    pub fn retry(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalRetry) -> Self {
        self.retry = Some(value);
        self
    }

    pub fn status(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn previous_status(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalPreviousStatus) -> Self {
        self.previous_status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentDismissal`].
    /// This method will fail if any of the following fields are not set:
    /// - [`previous_status`](QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalBuilder::previous_status)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentDismissal, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentDismissal {
            dismissal: self.dismissal,
            recovery_evidence: self.recovery_evidence,
            recurrence: self.recurrence,
            retry: self.retry,
            status: self.status,
            previous_status: self.previous_status.ok_or_else(|| BuildError::missing_field("previous_status"))?,
        })
    }
}
