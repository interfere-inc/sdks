pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TraceActivityItemsItem {
    pub kind: TraceActivityItemsItemKind,
    #[serde(rename = "timeUnixNano")]
    #[serde(default)]
    pub time_unix_nano: UInt64String,
    #[serde(rename = "traceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
    #[serde(rename = "spanId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub span_id: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "serviceName")]
    #[serde(default)]
    pub service_name: String,
    #[serde(rename = "actionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_id: Option<String>,
    #[serde(rename = "durationMs")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub duration_ms: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl TraceActivityItemsItem {
    pub fn builder() -> TraceActivityItemsItemBuilder {
        <TraceActivityItemsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TraceActivityItemsItemBuilder {
    kind: Option<TraceActivityItemsItemKind>,
    time_unix_nano: Option<UInt64String>,
    trace_id: Option<String>,
    span_id: Option<String>,
    name: Option<String>,
    service_name: Option<String>,
    action_id: Option<String>,
    duration_ms: Option<f64>,
    status: Option<String>,
}

impl TraceActivityItemsItemBuilder {
    pub fn kind(mut self, value: TraceActivityItemsItemKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn time_unix_nano(mut self, value: UInt64String) -> Self {
        self.time_unix_nano = Some(value);
        self
    }

    pub fn trace_id(mut self, value: impl Into<String>) -> Self {
        self.trace_id = Some(value.into());
        self
    }

    pub fn span_id(mut self, value: impl Into<String>) -> Self {
        self.span_id = Some(value.into());
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

    pub fn action_id(mut self, value: impl Into<String>) -> Self {
        self.action_id = Some(value.into());
        self
    }

    pub fn duration_ms(mut self, value: f64) -> Self {
        self.duration_ms = Some(value);
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TraceActivityItemsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](TraceActivityItemsItemBuilder::kind)
    /// - [`time_unix_nano`](TraceActivityItemsItemBuilder::time_unix_nano)
    /// - [`name`](TraceActivityItemsItemBuilder::name)
    /// - [`service_name`](TraceActivityItemsItemBuilder::service_name)
    /// - [`duration_ms`](TraceActivityItemsItemBuilder::duration_ms)
    pub fn build(self) -> Result<TraceActivityItemsItem, BuildError> {
        Ok(TraceActivityItemsItem {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            time_unix_nano: self.time_unix_nano.ok_or_else(|| BuildError::missing_field("time_unix_nano"))?,
            trace_id: self.trace_id,
            span_id: self.span_id,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            service_name: self.service_name.ok_or_else(|| BuildError::missing_field("service_name"))?,
            action_id: self.action_id,
            duration_ms: self.duration_ms.ok_or_else(|| BuildError::missing_field("duration_ms"))?,
            status: self.status,
        })
    }
}
