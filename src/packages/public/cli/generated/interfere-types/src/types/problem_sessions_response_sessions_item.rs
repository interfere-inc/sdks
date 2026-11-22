pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProblemSessionsResponseSessionsItem {
    #[serde(rename = "anonymousNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub anonymous_number: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer: Option<ProblemSessionsResponseSessionsItemConsumer>,
    #[serde(rename = "errorCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub error_count: Option<f64>,
    #[serde(rename = "eventCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub event_count: Option<f64>,
    /// Milliseconds since the Unix epoch in UTC.
    #[serde(rename = "firstEventAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_event_at: Option<i64>,
    /// Milliseconds since the Unix epoch in UTC.
    #[serde(rename = "firstOccurrenceAt")]
    #[serde(default)]
    pub first_occurrence_at: i64,
    /// Milliseconds since the Unix epoch in UTC.
    #[serde(rename = "lastEventAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_event_at: Option<i64>,
    #[serde(rename = "ownerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(rename = "replayDurationMs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub replay_duration_ms: Option<f64>,
    #[serde(rename = "replayOffsetMs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub replay_offset_ms: Option<f64>,
    /// Milliseconds since the Unix epoch in UTC.
    #[serde(rename = "replayStartedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replay_started_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub sequence: Option<f64>,
    #[serde(rename = "sessionId")]
    #[serde(default)]
    pub session_id: String,
    #[serde(rename = "sessionRecordId")]
    #[serde(default)]
    pub session_record_id: String,
    #[serde(rename = "surfaceId")]
    pub surface_id: ProblemSessionsResponseSessionsItemSurfaceId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<String>>,
}

impl ProblemSessionsResponseSessionsItem {
    pub fn builder() -> ProblemSessionsResponseSessionsItemBuilder {
        <ProblemSessionsResponseSessionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProblemSessionsResponseSessionsItemBuilder {
    anonymous_number: Option<f64>,
    browser: Option<String>,
    consumer: Option<ProblemSessionsResponseSessionsItemConsumer>,
    error_count: Option<f64>,
    event_count: Option<f64>,
    first_event_at: Option<i64>,
    first_occurrence_at: Option<i64>,
    last_event_at: Option<i64>,
    owner_id: Option<String>,
    platform: Option<String>,
    replay_duration_ms: Option<f64>,
    replay_offset_ms: Option<f64>,
    replay_started_at: Option<i64>,
    sequence: Option<f64>,
    session_id: Option<String>,
    session_record_id: Option<String>,
    surface_id: Option<ProblemSessionsResponseSessionsItemSurfaceId>,
    urls: Option<Vec<String>>,
}

impl ProblemSessionsResponseSessionsItemBuilder {
    pub fn anonymous_number(mut self, value: f64) -> Self {
        self.anonymous_number = Some(value);
        self
    }

    pub fn browser(mut self, value: impl Into<String>) -> Self {
        self.browser = Some(value.into());
        self
    }

    pub fn consumer(mut self, value: ProblemSessionsResponseSessionsItemConsumer) -> Self {
        self.consumer = Some(value);
        self
    }

    pub fn error_count(mut self, value: f64) -> Self {
        self.error_count = Some(value);
        self
    }

    pub fn event_count(mut self, value: f64) -> Self {
        self.event_count = Some(value);
        self
    }

    pub fn first_event_at(mut self, value: i64) -> Self {
        self.first_event_at = Some(value);
        self
    }

    pub fn first_occurrence_at(mut self, value: i64) -> Self {
        self.first_occurrence_at = Some(value);
        self
    }

    pub fn last_event_at(mut self, value: i64) -> Self {
        self.last_event_at = Some(value);
        self
    }

    pub fn owner_id(mut self, value: impl Into<String>) -> Self {
        self.owner_id = Some(value.into());
        self
    }

    pub fn platform(mut self, value: impl Into<String>) -> Self {
        self.platform = Some(value.into());
        self
    }

    pub fn replay_duration_ms(mut self, value: f64) -> Self {
        self.replay_duration_ms = Some(value);
        self
    }

    pub fn replay_offset_ms(mut self, value: f64) -> Self {
        self.replay_offset_ms = Some(value);
        self
    }

    pub fn replay_started_at(mut self, value: i64) -> Self {
        self.replay_started_at = Some(value);
        self
    }

    pub fn sequence(mut self, value: f64) -> Self {
        self.sequence = Some(value);
        self
    }

    pub fn session_id(mut self, value: impl Into<String>) -> Self {
        self.session_id = Some(value.into());
        self
    }

    pub fn session_record_id(mut self, value: impl Into<String>) -> Self {
        self.session_record_id = Some(value.into());
        self
    }

    pub fn surface_id(mut self, value: ProblemSessionsResponseSessionsItemSurfaceId) -> Self {
        self.surface_id = Some(value);
        self
    }

    pub fn urls(mut self, value: Vec<String>) -> Self {
        self.urls = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ProblemSessionsResponseSessionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`first_occurrence_at`](ProblemSessionsResponseSessionsItemBuilder::first_occurrence_at)
    /// - [`session_id`](ProblemSessionsResponseSessionsItemBuilder::session_id)
    /// - [`session_record_id`](ProblemSessionsResponseSessionsItemBuilder::session_record_id)
    /// - [`surface_id`](ProblemSessionsResponseSessionsItemBuilder::surface_id)
    pub fn build(self) -> Result<ProblemSessionsResponseSessionsItem, BuildError> {
        Ok(ProblemSessionsResponseSessionsItem {
            anonymous_number: self.anonymous_number,
            browser: self.browser,
            consumer: self.consumer,
            error_count: self.error_count,
            event_count: self.event_count,
            first_event_at: self.first_event_at,
            first_occurrence_at: self.first_occurrence_at.ok_or_else(|| BuildError::missing_field("first_occurrence_at"))?,
            last_event_at: self.last_event_at,
            owner_id: self.owner_id,
            platform: self.platform,
            replay_duration_ms: self.replay_duration_ms,
            replay_offset_ms: self.replay_offset_ms,
            replay_started_at: self.replay_started_at,
            sequence: self.sequence,
            session_id: self.session_id.ok_or_else(|| BuildError::missing_field("session_id"))?,
            session_record_id: self.session_record_id.ok_or_else(|| BuildError::missing_field("session_record_id"))?,
            surface_id: self.surface_id.ok_or_else(|| BuildError::missing_field("surface_id"))?,
            urls: self.urls,
        })
    }
}
