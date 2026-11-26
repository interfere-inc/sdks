pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentElevenFixReleaseId {
    #[serde(default)]
    pub version: String,
    #[serde(rename = "fixReleaseId")]
    #[serde(default)]
    pub fix_release_id: String,
    #[serde(rename = "recurredOnReleaseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurred_on_release_id: Option<String>,
    #[serde(rename = "findingId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentElevenFixReleaseIdRecurrence>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentElevenFixReleaseId {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentElevenFixReleaseIdBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentElevenFixReleaseIdBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentElevenFixReleaseIdBuilder {
    version: Option<String>,
    fix_release_id: Option<String>,
    recurred_on_release_id: Option<String>,
    finding_id: Option<String>,
    recurrence: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentElevenFixReleaseIdRecurrence>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentElevenFixReleaseIdBuilder {
    pub fn version(mut self, value: impl Into<String>) -> Self {
        self.version = Some(value.into());
        self
    }

    pub fn fix_release_id(mut self, value: impl Into<String>) -> Self {
        self.fix_release_id = Some(value.into());
        self
    }

    pub fn recurred_on_release_id(mut self, value: impl Into<String>) -> Self {
        self.recurred_on_release_id = Some(value.into());
        self
    }

    pub fn finding_id(mut self, value: impl Into<String>) -> Self {
        self.finding_id = Some(value.into());
        self
    }

    pub fn recurrence(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentElevenFixReleaseIdRecurrence) -> Self {
        self.recurrence = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentElevenFixReleaseId`].
    /// This method will fail if any of the following fields are not set:
    /// - [`version`](QueryProblemsGetByIdResponseTimelineEventsItemContentElevenFixReleaseIdBuilder::version)
    /// - [`fix_release_id`](QueryProblemsGetByIdResponseTimelineEventsItemContentElevenFixReleaseIdBuilder::fix_release_id)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentElevenFixReleaseId, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentElevenFixReleaseId {
            version: self.version.ok_or_else(|| BuildError::missing_field("version"))?,
            fix_release_id: self.fix_release_id.ok_or_else(|| BuildError::missing_field("fix_release_id"))?,
            recurred_on_release_id: self.recurred_on_release_id,
            finding_id: self.finding_id,
            recurrence: self.recurrence,
        })
    }
}
