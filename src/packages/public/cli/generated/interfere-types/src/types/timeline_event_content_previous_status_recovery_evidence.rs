pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TimelineEventContentPreviousStatusRecoveryEvidence {
    #[serde(default)]
    pub assessment: String,
    #[serde(rename = "sessionIds")]
    #[serde(default)]
    pub session_ids: Vec<String>,
    #[serde(default)]
    pub reviewed: bool,
}

impl TimelineEventContentPreviousStatusRecoveryEvidence {
    pub fn builder() -> TimelineEventContentPreviousStatusRecoveryEvidenceBuilder {
        <TimelineEventContentPreviousStatusRecoveryEvidenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentPreviousStatusRecoveryEvidenceBuilder {
    assessment: Option<String>,
    session_ids: Option<Vec<String>>,
    reviewed: Option<bool>,
}

impl TimelineEventContentPreviousStatusRecoveryEvidenceBuilder {
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

    /// Consumes the builder and constructs a [`TimelineEventContentPreviousStatusRecoveryEvidence`].
    /// This method will fail if any of the following fields are not set:
    /// - [`assessment`](TimelineEventContentPreviousStatusRecoveryEvidenceBuilder::assessment)
    /// - [`session_ids`](TimelineEventContentPreviousStatusRecoveryEvidenceBuilder::session_ids)
    /// - [`reviewed`](TimelineEventContentPreviousStatusRecoveryEvidenceBuilder::reviewed)
    pub fn build(self) -> Result<TimelineEventContentPreviousStatusRecoveryEvidence, BuildError> {
        Ok(TimelineEventContentPreviousStatusRecoveryEvidence {
            assessment: self.assessment.ok_or_else(|| BuildError::missing_field("assessment"))?,
            session_ids: self.session_ids.ok_or_else(|| BuildError::missing_field("session_ids"))?,
            reviewed: self.reviewed.ok_or_else(|| BuildError::missing_field("reviewed"))?,
        })
    }
}
