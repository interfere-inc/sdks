pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TimelineEventContentElevenFixReleaseIdRecurrence {
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub since: f64,
    #[serde(rename = "fixReleaseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fix_release_id: Option<String>,
    #[serde(default)]
    pub recurred: Vec<TimelineEventContentElevenFixReleaseIdRecurrenceRecurredItem>,
    #[serde(rename = "recurredCount")]
    #[serde(default)]
    pub recurred_count: i64,
    #[serde(default)]
    pub stopped: Vec<TimelineEventContentElevenFixReleaseIdRecurrenceStoppedItem>,
    #[serde(rename = "stoppedCount")]
    #[serde(default)]
    pub stopped_count: i64,
    #[serde(default)]
    pub summary: String,
}

impl TimelineEventContentElevenFixReleaseIdRecurrence {
    pub fn builder() -> TimelineEventContentElevenFixReleaseIdRecurrenceBuilder {
        <TimelineEventContentElevenFixReleaseIdRecurrenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentElevenFixReleaseIdRecurrenceBuilder {
    version: Option<String>,
    since: Option<f64>,
    fix_release_id: Option<String>,
    recurred: Option<Vec<TimelineEventContentElevenFixReleaseIdRecurrenceRecurredItem>>,
    recurred_count: Option<i64>,
    stopped: Option<Vec<TimelineEventContentElevenFixReleaseIdRecurrenceStoppedItem>>,
    stopped_count: Option<i64>,
    summary: Option<String>,
}

impl TimelineEventContentElevenFixReleaseIdRecurrenceBuilder {
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

    pub fn recurred(mut self, value: Vec<TimelineEventContentElevenFixReleaseIdRecurrenceRecurredItem>) -> Self {
        self.recurred = Some(value);
        self
    }

    pub fn recurred_count(mut self, value: i64) -> Self {
        self.recurred_count = Some(value);
        self
    }

    pub fn stopped(mut self, value: Vec<TimelineEventContentElevenFixReleaseIdRecurrenceStoppedItem>) -> Self {
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

    /// Consumes the builder and constructs a [`TimelineEventContentElevenFixReleaseIdRecurrence`].
    /// This method will fail if any of the following fields are not set:
    /// - [`version`](TimelineEventContentElevenFixReleaseIdRecurrenceBuilder::version)
    /// - [`since`](TimelineEventContentElevenFixReleaseIdRecurrenceBuilder::since)
    /// - [`recurred`](TimelineEventContentElevenFixReleaseIdRecurrenceBuilder::recurred)
    /// - [`recurred_count`](TimelineEventContentElevenFixReleaseIdRecurrenceBuilder::recurred_count)
    /// - [`stopped`](TimelineEventContentElevenFixReleaseIdRecurrenceBuilder::stopped)
    /// - [`stopped_count`](TimelineEventContentElevenFixReleaseIdRecurrenceBuilder::stopped_count)
    /// - [`summary`](TimelineEventContentElevenFixReleaseIdRecurrenceBuilder::summary)
    pub fn build(self) -> Result<TimelineEventContentElevenFixReleaseIdRecurrence, BuildError> {
        Ok(TimelineEventContentElevenFixReleaseIdRecurrence {
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
