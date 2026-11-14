pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for traceActivity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TraceActivityQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// occurredAt is required: pass the trace's firstAtUnixNano from GET /v3/findings/{findingId}/traces
    #[serde(rename = "occurredAt")]
    pub occurred_at: TraceActivityRequestOccurredAt,
}

impl TraceActivityQueryRequest {
    pub fn builder() -> TraceActivityQueryRequestBuilder {
        <TraceActivityQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TraceActivityQueryRequestBuilder {
    limit: Option<i64>,
    occurred_at: Option<TraceActivityRequestOccurredAt>,
}

impl TraceActivityQueryRequestBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn occurred_at(mut self, value: TraceActivityRequestOccurredAt) -> Self {
        self.occurred_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TraceActivityQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`occurred_at`](TraceActivityQueryRequestBuilder::occurred_at)
    pub fn build(self) -> Result<TraceActivityQueryRequest, BuildError> {
        Ok(TraceActivityQueryRequest {
            limit: self.limit,
            occurred_at: self.occurred_at.ok_or_else(|| BuildError::missing_field("occurred_at"))?,
        })
    }
}

