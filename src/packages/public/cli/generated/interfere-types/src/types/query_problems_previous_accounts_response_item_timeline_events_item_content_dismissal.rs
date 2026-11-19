pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissal {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dismissal: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalDismissal>,
    #[serde(rename = "recoveryEvidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_evidence: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalRecoveryEvidence>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalRetry>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalStatus>,
    #[serde(rename = "previousStatus")]
    pub previous_status: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalPreviousStatus,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissal {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalBuilder {
    dismissal: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalDismissal>,
    recovery_evidence: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalRecoveryEvidence>,
    retry: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalRetry>,
    status: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalStatus>,
    previous_status: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalPreviousStatus>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalBuilder {
    pub fn dismissal(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalDismissal) -> Self {
        self.dismissal = Some(value);
        self
    }

    pub fn recovery_evidence(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalRecoveryEvidence) -> Self {
        self.recovery_evidence = Some(value);
        self
    }

    pub fn retry(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalRetry) -> Self {
        self.retry = Some(value);
        self
    }

    pub fn status(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn previous_status(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalPreviousStatus) -> Self {
        self.previous_status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissal`].
    /// This method will fail if any of the following fields are not set:
    /// - [`previous_status`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalBuilder::previous_status)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissal, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissal {
            dismissal: self.dismissal,
            recovery_evidence: self.recovery_evidence,
            retry: self.retry,
            status: self.status,
            previous_status: self.previous_status.ok_or_else(|| BuildError::missing_field("previous_status"))?,
        })
    }
}
