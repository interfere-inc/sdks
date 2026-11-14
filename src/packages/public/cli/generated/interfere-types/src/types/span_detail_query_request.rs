pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for spanDetail
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SpanDetailQueryRequest {
    /// Optional endedAtUnixNano from GET /v3/findings/{findingId}/traces/{traceId}; narrows the read to the trace's extent within the window days
    #[serde(rename = "endedAtUnixNano")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ended_at_unix_nano: Option<String>,
    /// Optional startedAtUnixNano from GET /v3/findings/{findingId}/traces/{traceId}; narrows the read to the trace's extent within the window days
    #[serde(rename = "startedAtUnixNano")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at_unix_nano: Option<String>,
    /// windowEnd is required: pass windowEnd from GET /v3/findings/{findingId}/traces/{traceId}
    #[serde(rename = "windowEnd")]
    #[serde(default)]
    pub window_end: NaiveDate,
    /// windowStart is required: pass windowStart from GET /v3/findings/{findingId}/traces/{traceId}
    #[serde(rename = "windowStart")]
    #[serde(default)]
    pub window_start: NaiveDate,
}

impl SpanDetailQueryRequest {
    pub fn builder() -> SpanDetailQueryRequestBuilder {
        <SpanDetailQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SpanDetailQueryRequestBuilder {
    ended_at_unix_nano: Option<String>,
    started_at_unix_nano: Option<String>,
    window_end: Option<NaiveDate>,
    window_start: Option<NaiveDate>,
}

impl SpanDetailQueryRequestBuilder {
    pub fn ended_at_unix_nano(mut self, value: impl Into<String>) -> Self {
        self.ended_at_unix_nano = Some(value.into());
        self
    }

    pub fn started_at_unix_nano(mut self, value: impl Into<String>) -> Self {
        self.started_at_unix_nano = Some(value.into());
        self
    }

    pub fn window_end(mut self, value: NaiveDate) -> Self {
        self.window_end = Some(value);
        self
    }

    pub fn window_start(mut self, value: NaiveDate) -> Self {
        self.window_start = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SpanDetailQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`window_end`](SpanDetailQueryRequestBuilder::window_end)
    /// - [`window_start`](SpanDetailQueryRequestBuilder::window_start)
    pub fn build(self) -> Result<SpanDetailQueryRequest, BuildError> {
        Ok(SpanDetailQueryRequest {
            ended_at_unix_nano: self.ended_at_unix_nano,
            started_at_unix_nano: self.started_at_unix_nano,
            window_end: self.window_end.ok_or_else(|| BuildError::missing_field("window_end"))?,
            window_start: self.window_start.ok_or_else(|| BuildError::missing_field("window_start"))?,
        })
    }
}

