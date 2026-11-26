pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalRecurrence {
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub since: f64,
    #[serde(rename = "fixReleaseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fix_release_id: Option<String>,
    #[serde(default)]
    pub recurred: Vec<QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalRecurrenceRecurredItem>,
    #[serde(rename = "recurredCount")]
    #[serde(default)]
    pub recurred_count: i64,
    #[serde(default)]
    pub stopped: Vec<QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalRecurrenceStoppedItem>,
    #[serde(rename = "stoppedCount")]
    #[serde(default)]
    pub stopped_count: i64,
    #[serde(default)]
    pub summary: String,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalRecurrence {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalRecurrenceBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalRecurrenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalRecurrenceBuilder {
    version: Option<String>,
    since: Option<f64>,
    fix_release_id: Option<String>,
    recurred: Option<Vec<QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalRecurrenceRecurredItem>>,
    recurred_count: Option<i64>,
    stopped: Option<Vec<QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalRecurrenceStoppedItem>>,
    stopped_count: Option<i64>,
    summary: Option<String>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalRecurrenceBuilder {
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

    pub fn recurred(mut self, value: Vec<QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalRecurrenceRecurredItem>) -> Self {
        self.recurred = Some(value);
        self
    }

    pub fn recurred_count(mut self, value: i64) -> Self {
        self.recurred_count = Some(value);
        self
    }

    pub fn stopped(mut self, value: Vec<QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalRecurrenceStoppedItem>) -> Self {
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

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalRecurrence`].
    /// This method will fail if any of the following fields are not set:
    /// - [`version`](QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalRecurrenceBuilder::version)
    /// - [`since`](QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalRecurrenceBuilder::since)
    /// - [`recurred`](QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalRecurrenceBuilder::recurred)
    /// - [`recurred_count`](QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalRecurrenceBuilder::recurred_count)
    /// - [`stopped`](QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalRecurrenceBuilder::stopped)
    /// - [`stopped_count`](QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalRecurrenceBuilder::stopped_count)
    /// - [`summary`](QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalRecurrenceBuilder::summary)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalRecurrence, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalRecurrence {
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
