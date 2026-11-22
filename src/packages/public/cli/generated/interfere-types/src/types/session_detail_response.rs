pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SessionDetailResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser: Option<String>,
    #[serde(rename = "errorCount")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub error_count: f64,
    #[serde(rename = "eventCount")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub event_count: f64,
    #[serde(default)]
    pub events: Vec<SessionDetailResponseEventsItem>,
    /// Milliseconds since the Unix epoch in UTC.
    #[serde(rename = "firstEventAt")]
    #[serde(default)]
    pub first_event_at: i64,
    /// Milliseconds since the Unix epoch in UTC.
    #[serde(rename = "lastEventAt")]
    #[serde(default)]
    pub last_event_at: i64,
    #[serde(default)]
    pub navigations: Vec<SessionDetailResponseNavigationsItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
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
    pub surface_id: SessionDetailResponseSurfaceId,
}

impl SessionDetailResponse {
    pub fn builder() -> SessionDetailResponseBuilder {
        <SessionDetailResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SessionDetailResponseBuilder {
    browser: Option<String>,
    error_count: Option<f64>,
    event_count: Option<f64>,
    events: Option<Vec<SessionDetailResponseEventsItem>>,
    first_event_at: Option<i64>,
    last_event_at: Option<i64>,
    navigations: Option<Vec<SessionDetailResponseNavigationsItem>>,
    platform: Option<String>,
    sequence: Option<f64>,
    session_id: Option<String>,
    session_record_id: Option<String>,
    surface_id: Option<SessionDetailResponseSurfaceId>,
}

impl SessionDetailResponseBuilder {
    pub fn browser(mut self, value: impl Into<String>) -> Self {
        self.browser = Some(value.into());
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

    pub fn events(mut self, value: Vec<SessionDetailResponseEventsItem>) -> Self {
        self.events = Some(value);
        self
    }

    pub fn first_event_at(mut self, value: i64) -> Self {
        self.first_event_at = Some(value);
        self
    }

    pub fn last_event_at(mut self, value: i64) -> Self {
        self.last_event_at = Some(value);
        self
    }

    pub fn navigations(mut self, value: Vec<SessionDetailResponseNavigationsItem>) -> Self {
        self.navigations = Some(value);
        self
    }

    pub fn platform(mut self, value: impl Into<String>) -> Self {
        self.platform = Some(value.into());
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

    pub fn surface_id(mut self, value: SessionDetailResponseSurfaceId) -> Self {
        self.surface_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SessionDetailResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`error_count`](SessionDetailResponseBuilder::error_count)
    /// - [`event_count`](SessionDetailResponseBuilder::event_count)
    /// - [`events`](SessionDetailResponseBuilder::events)
    /// - [`first_event_at`](SessionDetailResponseBuilder::first_event_at)
    /// - [`last_event_at`](SessionDetailResponseBuilder::last_event_at)
    /// - [`navigations`](SessionDetailResponseBuilder::navigations)
    /// - [`session_id`](SessionDetailResponseBuilder::session_id)
    /// - [`session_record_id`](SessionDetailResponseBuilder::session_record_id)
    /// - [`surface_id`](SessionDetailResponseBuilder::surface_id)
    pub fn build(self) -> Result<SessionDetailResponse, BuildError> {
        Ok(SessionDetailResponse {
            browser: self.browser,
            error_count: self.error_count.ok_or_else(|| BuildError::missing_field("error_count"))?,
            event_count: self.event_count.ok_or_else(|| BuildError::missing_field("event_count"))?,
            events: self.events.ok_or_else(|| BuildError::missing_field("events"))?,
            first_event_at: self.first_event_at.ok_or_else(|| BuildError::missing_field("first_event_at"))?,
            last_event_at: self.last_event_at.ok_or_else(|| BuildError::missing_field("last_event_at"))?,
            navigations: self.navigations.ok_or_else(|| BuildError::missing_field("navigations"))?,
            platform: self.platform,
            sequence: self.sequence,
            session_id: self.session_id.ok_or_else(|| BuildError::missing_field("session_id"))?,
            session_record_id: self.session_record_id.ok_or_else(|| BuildError::missing_field("session_record_id"))?,
            surface_id: self.surface_id.ok_or_else(|| BuildError::missing_field("surface_id"))?,
        })
    }
}
