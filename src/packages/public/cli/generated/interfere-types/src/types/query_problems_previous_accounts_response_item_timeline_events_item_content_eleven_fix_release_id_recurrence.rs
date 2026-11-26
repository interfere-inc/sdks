pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenFixReleaseIdRecurrence {
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub since: f64,
    #[serde(rename = "fixReleaseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fix_release_id: Option<String>,
    #[serde(default)]
    pub recurred: Vec<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenFixReleaseIdRecurrenceRecurredItem>,
    #[serde(rename = "recurredCount")]
    #[serde(default)]
    pub recurred_count: i64,
    #[serde(default)]
    pub stopped: Vec<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenFixReleaseIdRecurrenceStoppedItem>,
    #[serde(rename = "stoppedCount")]
    #[serde(default)]
    pub stopped_count: i64,
    #[serde(default)]
    pub summary: String,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenFixReleaseIdRecurrence {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenFixReleaseIdRecurrenceBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenFixReleaseIdRecurrenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenFixReleaseIdRecurrenceBuilder {
    version: Option<String>,
    since: Option<f64>,
    fix_release_id: Option<String>,
    recurred: Option<Vec<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenFixReleaseIdRecurrenceRecurredItem>>,
    recurred_count: Option<i64>,
    stopped: Option<Vec<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenFixReleaseIdRecurrenceStoppedItem>>,
    stopped_count: Option<i64>,
    summary: Option<String>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenFixReleaseIdRecurrenceBuilder {
    pub fn version(mut self, value: impl Into<String>) -> Self {
        self.version = Some(value.into());
        self
    }

    pub fn since(mut self, value: f64) -> Self {
        self.since = Some(value);
        self
    }

    pub fn fix_release_id(mut self, value: impl Into<String>) -> Self {
        self.fix_release_id = Some(value.into());
        self
    }

    pub fn recurred(mut self, value: Vec<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenFixReleaseIdRecurrenceRecurredItem>) -> Self {
        self.recurred = Some(value);
        self
    }

    pub fn recurred_count(mut self, value: i64) -> Self {
        self.recurred_count = Some(value);
        self
    }

    pub fn stopped(mut self, value: Vec<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenFixReleaseIdRecurrenceStoppedItem>) -> Self {
        self.stopped = Some(value);
        self
    }

    pub fn stopped_count(mut self, value: i64) -> Self {
        self.stopped_count = Some(value);
        self
    }

    pub fn summary(mut self, value: impl Into<String>) -> Self {
        self.summary = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenFixReleaseIdRecurrence`].
    /// This method will fail if any of the following fields are not set:
    /// - [`version`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenFixReleaseIdRecurrenceBuilder::version)
    /// - [`since`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenFixReleaseIdRecurrenceBuilder::since)
    /// - [`recurred`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenFixReleaseIdRecurrenceBuilder::recurred)
    /// - [`recurred_count`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenFixReleaseIdRecurrenceBuilder::recurred_count)
    /// - [`stopped`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenFixReleaseIdRecurrenceBuilder::stopped)
    /// - [`stopped_count`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenFixReleaseIdRecurrenceBuilder::stopped_count)
    /// - [`summary`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenFixReleaseIdRecurrenceBuilder::summary)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenFixReleaseIdRecurrence, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentElevenFixReleaseIdRecurrence {
            version: self.version.ok_or_else(|| BuildError::missing_field("version"))?,
            since: self.since.ok_or_else(|| BuildError::missing_field("since"))?,
            fix_release_id: self.fix_release_id,
            recurred: self.recurred.ok_or_else(|| BuildError::missing_field("recurred"))?,
            recurred_count: self.recurred_count.ok_or_else(|| BuildError::missing_field("recurred_count"))?,
            stopped: self.stopped.ok_or_else(|| BuildError::missing_field("stopped"))?,
            stopped_count: self.stopped_count.ok_or_else(|| BuildError::missing_field("stopped_count"))?,
            summary: self.summary.ok_or_else(|| BuildError::missing_field("summary"))?,
        })
    }
}
