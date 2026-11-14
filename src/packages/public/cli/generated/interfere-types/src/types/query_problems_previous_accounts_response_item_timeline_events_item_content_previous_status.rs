pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentPreviousStatus {
    #[serde(rename = "recoveryEvidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_evidence: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentPreviousStatusRecoveryEvidence>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentPreviousStatusRetry>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentPreviousStatusStatus>,
    #[serde(rename = "previousStatus")]
    pub previous_status: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentPreviousStatusPreviousStatus,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentPreviousStatus {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentPreviousStatusBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentPreviousStatusBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentPreviousStatusBuilder {
    recovery_evidence: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentPreviousStatusRecoveryEvidence>,
    retry: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentPreviousStatusRetry>,
    status: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentPreviousStatusStatus>,
    previous_status: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentPreviousStatusPreviousStatus>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentPreviousStatusBuilder {
    pub fn recovery_evidence(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentPreviousStatusRecoveryEvidence) -> Self {
        self.recovery_evidence = Some(value);
        self
    }

    pub fn retry(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentPreviousStatusRetry) -> Self {
        self.retry = Some(value);
        self
    }

    pub fn status(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentPreviousStatusStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn previous_status(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentPreviousStatusPreviousStatus) -> Self {
        self.previous_status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentPreviousStatus`].
    /// This method will fail if any of the following fields are not set:
    /// - [`previous_status`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentPreviousStatusBuilder::previous_status)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentPreviousStatus, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentPreviousStatus {
            recovery_evidence: self.recovery_evidence,
            retry: self.retry,
            status: self.status,
            previous_status: self.previous_status.ok_or_else(|| BuildError::missing_field("previous_status"))?,
        })
    }
}
