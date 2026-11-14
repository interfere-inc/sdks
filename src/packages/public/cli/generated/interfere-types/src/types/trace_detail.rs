pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TraceDetail {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser: Option<String>,
    #[serde(rename = "danglingParentCount")]
    #[serde(default)]
    pub dangling_parent_count: i64,
    #[serde(rename = "endedAtUnixNano")]
    #[serde(default)]
    pub ended_at_unix_nano: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(rename = "sessionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(rename = "deviceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(rename = "rootCount")]
    #[serde(default)]
    pub root_count: i64,
    #[serde(default)]
    pub spans: Vec<TraceSpan>,
    #[serde(rename = "startedAtUnixNano")]
    #[serde(default)]
    pub started_at_unix_nano: String,
    #[serde(rename = "traceId")]
    #[serde(default)]
    pub trace_id: String,
    #[serde(default)]
    pub truncated: bool,
    #[serde(rename = "wallDurationMs")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub wall_duration_ms: f64,
    #[serde(rename = "windowEnd")]
    #[serde(default)]
    pub window_end: String,
    #[serde(rename = "windowStart")]
    #[serde(default)]
    pub window_start: String,
}

impl TraceDetail {
    pub fn builder() -> TraceDetailBuilder {
        <TraceDetailBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TraceDetailBuilder {
    browser: Option<String>,
    dangling_parent_count: Option<i64>,
    ended_at_unix_nano: Option<String>,
    platform: Option<String>,
    session_id: Option<String>,
    device_id: Option<String>,
    root_count: Option<i64>,
    spans: Option<Vec<TraceSpan>>,
    started_at_unix_nano: Option<String>,
    trace_id: Option<String>,
    truncated: Option<bool>,
    wall_duration_ms: Option<f64>,
    window_end: Option<String>,
    window_start: Option<String>,
}

impl TraceDetailBuilder {
    pub fn browser(mut self, value: impl Into<String>) -> Self {
        self.browser = Some(value.into());
        self
    }

    pub fn dangling_parent_count(mut self, value: i64) -> Self {
        self.dangling_parent_count = Some(value);
        self
    }

    pub fn ended_at_unix_nano(mut self, value: impl Into<String>) -> Self {
        self.ended_at_unix_nano = Some(value.into());
        self
    }

    pub fn platform(mut self, value: impl Into<String>) -> Self {
        self.platform = Some(value.into());
        self
    }

    pub fn session_id(mut self, value: impl Into<String>) -> Self {
        self.session_id = Some(value.into());
        self
    }

    pub fn device_id(mut self, value: impl Into<String>) -> Self {
        self.device_id = Some(value.into());
        self
    }

    pub fn root_count(mut self, value: i64) -> Self {
        self.root_count = Some(value);
        self
    }

    pub fn spans(mut self, value: Vec<TraceSpan>) -> Self {
        self.spans = Some(value);
        self
    }

    pub fn started_at_unix_nano(mut self, value: impl Into<String>) -> Self {
        self.started_at_unix_nano = Some(value.into());
        self
    }

    pub fn trace_id(mut self, value: impl Into<String>) -> Self {
        self.trace_id = Some(value.into());
        self
    }

    pub fn truncated(mut self, value: bool) -> Self {
        self.truncated = Some(value);
        self
    }

    pub fn wall_duration_ms(mut self, value: f64) -> Self {
        self.wall_duration_ms = Some(value);
        self
    }

    pub fn window_end(mut self, value: impl Into<String>) -> Self {
        self.window_end = Some(value.into());
        self
    }

    pub fn window_start(mut self, value: impl Into<String>) -> Self {
        self.window_start = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TraceDetail`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dangling_parent_count`](TraceDetailBuilder::dangling_parent_count)
    /// - [`ended_at_unix_nano`](TraceDetailBuilder::ended_at_unix_nano)
    /// - [`root_count`](TraceDetailBuilder::root_count)
    /// - [`spans`](TraceDetailBuilder::spans)
    /// - [`started_at_unix_nano`](TraceDetailBuilder::started_at_unix_nano)
    /// - [`trace_id`](TraceDetailBuilder::trace_id)
    /// - [`truncated`](TraceDetailBuilder::truncated)
    /// - [`wall_duration_ms`](TraceDetailBuilder::wall_duration_ms)
    /// - [`window_end`](TraceDetailBuilder::window_end)
    /// - [`window_start`](TraceDetailBuilder::window_start)
    pub fn build(self) -> Result<TraceDetail, BuildError> {
        Ok(TraceDetail {
            browser: self.browser,
            dangling_parent_count: self.dangling_parent_count.ok_or_else(|| BuildError::missing_field("dangling_parent_count"))?,
            ended_at_unix_nano: self.ended_at_unix_nano.ok_or_else(|| BuildError::missing_field("ended_at_unix_nano"))?,
            platform: self.platform,
            session_id: self.session_id,
            device_id: self.device_id,
            root_count: self.root_count.ok_or_else(|| BuildError::missing_field("root_count"))?,
            spans: self.spans.ok_or_else(|| BuildError::missing_field("spans"))?,
            started_at_unix_nano: self.started_at_unix_nano.ok_or_else(|| BuildError::missing_field("started_at_unix_nano"))?,
            trace_id: self.trace_id.ok_or_else(|| BuildError::missing_field("trace_id"))?,
            truncated: self.truncated.ok_or_else(|| BuildError::missing_field("truncated"))?,
            wall_duration_ms: self.wall_duration_ms.ok_or_else(|| BuildError::missing_field("wall_duration_ms"))?,
            window_end: self.window_end.ok_or_else(|| BuildError::missing_field("window_end"))?,
            window_start: self.window_start.ok_or_else(|| BuildError::missing_field("window_start"))?,
        })
    }
}
