pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentPreviousStatus {
    #[serde(rename = "recoveryEvidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_evidence: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentPreviousStatusRecoveryEvidence>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentPreviousStatusRetry>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentPreviousStatusStatus>,
    #[serde(rename = "previousStatus")]
    pub previous_status: QueryProblemsGetByIdResponseTimelineEventsItemContentPreviousStatusPreviousStatus,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentPreviousStatus {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentPreviousStatusBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentPreviousStatusBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentPreviousStatusBuilder {
    recovery_evidence: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentPreviousStatusRecoveryEvidence>,
    retry: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentPreviousStatusRetry>,
    status: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentPreviousStatusStatus>,
    previous_status: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentPreviousStatusPreviousStatus>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentPreviousStatusBuilder {
    pub fn recovery_evidence(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentPreviousStatusRecoveryEvidence) -> Self {
        self.recovery_evidence = Some(value);
        self
    }

    pub fn retry(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentPreviousStatusRetry) -> Self {
        self.retry = Some(value);
        self
    }

    pub fn status(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentPreviousStatusStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn previous_status(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentPreviousStatusPreviousStatus) -> Self {
        self.previous_status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentPreviousStatus`].
    /// This method will fail if any of the following fields are not set:
    /// - [`previous_status`](QueryProblemsGetByIdResponseTimelineEventsItemContentPreviousStatusBuilder::previous_status)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentPreviousStatus, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentPreviousStatus {
            recovery_evidence: self.recovery_evidence,
            retry: self.retry,
            status: self.status,
            previous_status: self.previous_status.ok_or_else(|| BuildError::missing_field("previous_status"))?,
        })
    }
}
