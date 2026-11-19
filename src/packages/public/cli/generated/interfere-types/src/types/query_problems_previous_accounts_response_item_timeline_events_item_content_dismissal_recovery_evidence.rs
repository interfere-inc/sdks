pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalRecoveryEvidence {
    #[serde(default)]
    pub assessment: String,
    #[serde(rename = "sessionIds")]
    #[serde(default)]
    pub session_ids: Vec<String>,
    #[serde(default)]
    pub reviewed: bool,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalRecoveryEvidence {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalRecoveryEvidenceBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalRecoveryEvidenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalRecoveryEvidenceBuilder {
    assessment: Option<String>,
    session_ids: Option<Vec<String>>,
    reviewed: Option<bool>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalRecoveryEvidenceBuilder {
    pub fn assessment(mut self, value: impl Into<String>) -> Self {
        self.assessment = Some(value.into());
        self
    }

    pub fn session_ids(mut self, value: Vec<String>) -> Self {
        self.session_ids = Some(value);
        self
    }

    pub fn reviewed(mut self, value: bool) -> Self {
        self.reviewed = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalRecoveryEvidence`].
    /// This method will fail if any of the following fields are not set:
    /// - [`assessment`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalRecoveryEvidenceBuilder::assessment)
    /// - [`session_ids`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalRecoveryEvidenceBuilder::session_ids)
    /// - [`reviewed`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalRecoveryEvidenceBuilder::reviewed)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalRecoveryEvidence, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalRecoveryEvidence {
            assessment: self.assessment.ok_or_else(|| BuildError::missing_field("assessment"))?,
            session_ids: self.session_ids.ok_or_else(|| BuildError::missing_field("session_ids"))?,
            reviewed: self.reviewed.ok_or_else(|| BuildError::missing_field("reviewed"))?,
        })
    }
}
