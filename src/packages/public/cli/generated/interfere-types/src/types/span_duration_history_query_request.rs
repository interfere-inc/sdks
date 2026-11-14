pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for spanDurationHistory
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SpanDurationHistoryQueryRequest {
    #[serde(default)]
    pub kind: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "serviceName")]
    #[serde(default)]
    pub service_name: String,
}

impl SpanDurationHistoryQueryRequest {
    pub fn builder() -> SpanDurationHistoryQueryRequestBuilder {
        <SpanDurationHistoryQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SpanDurationHistoryQueryRequestBuilder {
    kind: Option<String>,
    name: Option<String>,
    service_name: Option<String>,
}

impl SpanDurationHistoryQueryRequestBuilder {
    pub fn kind(mut self, value: impl Into<String>) -> Self {
        self.kind = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn service_name(mut self, value: impl Into<String>) -> Self {
        self.service_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SpanDurationHistoryQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](SpanDurationHistoryQueryRequestBuilder::kind)
    /// - [`name`](SpanDurationHistoryQueryRequestBuilder::name)
    /// - [`service_name`](SpanDurationHistoryQueryRequestBuilder::service_name)
    pub fn build(self) -> Result<SpanDurationHistoryQueryRequest, BuildError> {
        Ok(SpanDurationHistoryQueryRequest {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            service_name: self.service_name.ok_or_else(|| BuildError::missing_field("service_name"))?,
        })
    }
}

