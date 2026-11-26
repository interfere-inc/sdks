pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SessionSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser: Option<String>,
    #[serde(rename = "deviceId")]
    #[serde(default)]
    pub device_id: String,
    #[serde(rename = "endUserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_user_id: Option<String>,
    #[serde(rename = "errorCount")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub error_count: f64,
    #[serde(rename = "eventCount")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub event_count: f64,
    #[serde(default)]
    pub fingerprints: Vec<String>,
    /// Milliseconds since the Unix epoch in UTC.
    #[serde(rename = "firstEventAt")]
    #[serde(default)]
    pub first_event_at: i64,
    /// Milliseconds since the Unix epoch in UTC.
    #[serde(rename = "lastEventAt")]
    #[serde(default)]
    pub last_event_at: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(rename = "recordingKey")]
    #[serde(default)]
    pub recording_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub sequence: Option<f64>,
    #[serde(rename = "sessionId")]
    #[serde(default)]
    pub session_id: String,
    #[serde(rename = "surfaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface_id: Option<SessionSummarySurfaceId>,
}

impl SessionSummary {
    pub fn builder() -> SessionSummaryBuilder {
        <SessionSummaryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SessionSummaryBuilder {
    browser: Option<String>,
    device_id: Option<String>,
    end_user_id: Option<String>,
    error_count: Option<f64>,
    event_count: Option<f64>,
    fingerprints: Option<Vec<String>>,
    first_event_at: Option<i64>,
    last_event_at: Option<i64>,
    platform: Option<String>,
    recording_key: Option<String>,
    sequence: Option<f64>,
    session_id: Option<String>,
    surface_id: Option<SessionSummarySurfaceId>,
}

impl SessionSummaryBuilder {
    pub fn browser(mut self, value: impl Into<String>) -> Self {
        self.browser = Some(value.into());
        self
    }

    pub fn device_id(mut self, value: impl Into<String>) -> Self {
        self.device_id = Some(value.into());
        self
    }

    pub fn end_user_id(mut self, value: impl Into<String>) -> Self {
        self.end_user_id = Some(value.into());
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

    pub fn fingerprints(mut self, value: Vec<String>) -> Self {
        self.fingerprints = Some(value);
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

    pub fn platform(mut self, value: impl Into<String>) -> Self {
        self.platform = Some(value.into());
        self
    }

    pub fn recording_key(mut self, value: impl Into<String>) -> Self {
        self.recording_key = Some(value.into());
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

    pub fn surface_id(mut self, value: SessionSummarySurfaceId) -> Self {
        self.surface_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SessionSummary`].
    /// This method will fail if any of the following fields are not set:
    /// - [`device_id`](SessionSummaryBuilder::device_id)
    /// - [`error_count`](SessionSummaryBuilder::error_count)
    /// - [`event_count`](SessionSummaryBuilder::event_count)
    /// - [`fingerprints`](SessionSummaryBuilder::fingerprints)
    /// - [`first_event_at`](SessionSummaryBuilder::first_event_at)
    /// - [`last_event_at`](SessionSummaryBuilder::last_event_at)
    /// - [`recording_key`](SessionSummaryBuilder::recording_key)
    /// - [`session_id`](SessionSummaryBuilder::session_id)
    pub fn build(self) -> Result<SessionSummary, BuildError> {
        Ok(SessionSummary {
            browser: self.browser,
            device_id: self.device_id.ok_or_else(|| BuildError::missing_field("device_id"))?,
            end_user_id: self.end_user_id,
            error_count: self.error_count.ok_or_else(|| BuildError::missing_field("error_count"))?,
            event_count: self.event_count.ok_or_else(|| BuildError::missing_field("event_count"))?,
            fingerprints: self.fingerprints.ok_or_else(|| BuildError::missing_field("fingerprints"))?,
            first_event_at: self.first_event_at.ok_or_else(|| BuildError::missing_field("first_event_at"))?,
            last_event_at: self.last_event_at.ok_or_else(|| BuildError::missing_field("last_event_at"))?,
            platform: self.platform,
            recording_key: self.recording_key.ok_or_else(|| BuildError::missing_field("recording_key"))?,
            sequence: self.sequence,
            session_id: self.session_id.ok_or_else(|| BuildError::missing_field("session_id"))?,
            surface_id: self.surface_id,
        })
    }
}
