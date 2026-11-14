pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for traceDetail
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TraceDetailQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// occurredAt is required: pass the trace's firstAtUnixNano from GET /v3/findings/{findingId}/traces
    #[serde(rename = "occurredAt")]
    pub occurred_at: TraceDetailRequestOccurredAt,
}

impl TraceDetailQueryRequest {
    pub fn builder() -> TraceDetailQueryRequestBuilder {
        <TraceDetailQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TraceDetailQueryRequestBuilder {
    limit: Option<i64>,
    occurred_at: Option<TraceDetailRequestOccurredAt>,
}

impl TraceDetailQueryRequestBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn occurred_at(mut self, value: TraceDetailRequestOccurredAt) -> Self {
        self.occurred_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TraceDetailQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`occurred_at`](TraceDetailQueryRequestBuilder::occurred_at)
    pub fn build(self) -> Result<TraceDetailQueryRequest, BuildError> {
        Ok(TraceDetailQueryRequest {
            limit: self.limit,
            occurred_at: self.occurred_at.ok_or_else(|| BuildError::missing_field("occurred_at"))?,
        })
    }
}

