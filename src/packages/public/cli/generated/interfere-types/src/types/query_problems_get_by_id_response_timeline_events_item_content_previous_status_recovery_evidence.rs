pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentPreviousStatusRecoveryEvidence {
    #[serde(default)]
    pub assessment: String,
    #[serde(rename = "sessionIds")]
    #[serde(default)]
    pub session_ids: Vec<String>,
    #[serde(default)]
    pub reviewed: bool,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentPreviousStatusRecoveryEvidence {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentPreviousStatusRecoveryEvidenceBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentPreviousStatusRecoveryEvidenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentPreviousStatusRecoveryEvidenceBuilder {
    assessment: Option<String>,
    session_ids: Option<Vec<String>>,
    reviewed: Option<bool>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentPreviousStatusRecoveryEvidenceBuilder {
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

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentPreviousStatusRecoveryEvidence`].
    /// This method will fail if any of the following fields are not set:
    /// - [`assessment`](QueryProblemsGetByIdResponseTimelineEventsItemContentPreviousStatusRecoveryEvidenceBuilder::assessment)
    /// - [`session_ids`](QueryProblemsGetByIdResponseTimelineEventsItemContentPreviousStatusRecoveryEvidenceBuilder::session_ids)
    /// - [`reviewed`](QueryProblemsGetByIdResponseTimelineEventsItemContentPreviousStatusRecoveryEvidenceBuilder::reviewed)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentPreviousStatusRecoveryEvidence, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentPreviousStatusRecoveryEvidence {
            assessment: self.assessment.ok_or_else(|| BuildError::missing_field("assessment"))?,
            session_ids: self.session_ids.ok_or_else(|| BuildError::missing_field("session_ids"))?,
            reviewed: self.reviewed.ok_or_else(|| BuildError::missing_field("reviewed"))?,
        })
    }
}
