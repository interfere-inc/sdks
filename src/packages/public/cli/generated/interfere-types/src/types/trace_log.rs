pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TraceLog {
    #[serde(rename = "spanId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub span_id: Option<String>,
    #[serde(default)]
    pub timestamp: String,
    #[serde(rename = "severityNumber")]
    #[serde(default)]
    pub severity_number: i64,
    #[serde(rename = "severityText")]
    #[serde(default)]
    pub severity_text: String,
    #[serde(rename = "serviceName")]
    #[serde(default)]
    pub service_name: String,
    #[serde(default)]
    pub body: String,
    /// Log record attributes as a JSON object string; null when the source does not retain them
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<String>,
}

impl TraceLog {
    pub fn builder() -> TraceLogBuilder {
        <TraceLogBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TraceLogBuilder {
    span_id: Option<String>,
    timestamp: Option<String>,
    severity_number: Option<i64>,
    severity_text: Option<String>,
    service_name: Option<String>,
    body: Option<String>,
    attributes: Option<String>,
}

impl TraceLogBuilder {
    pub fn span_id(mut self, value: impl Into<String>) -> Self {
        self.span_id = Some(value.into());
        self
    }

    pub fn timestamp(mut self, value: impl Into<String>) -> Self {
        self.timestamp = Some(value.into());
        self
    }

    pub fn severity_number(mut self, value: i64) -> Self {
        self.severity_number = Some(value);
        self
    }

    pub fn severity_text(mut self, value: impl Into<String>) -> Self {
        self.severity_text = Some(value.into());
        self
    }

    pub fn service_name(mut self, value: impl Into<String>) -> Self {
        self.service_name = Some(value.into());
        self
    }

    pub fn body(mut self, value: impl Into<String>) -> Self {
        self.body = Some(value.into());
        self
    }

    pub fn attributes(mut self, value: impl Into<String>) -> Self {
        self.attributes = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TraceLog`].
    /// This method will fail if any of the following fields are not set:
    /// - [`timestamp`](TraceLogBuilder::timestamp)
    /// - [`severity_number`](TraceLogBuilder::severity_number)
    /// - [`severity_text`](TraceLogBuilder::severity_text)
    /// - [`service_name`](TraceLogBuilder::service_name)
    /// - [`body`](TraceLogBuilder::body)
    pub fn build(self) -> Result<TraceLog, BuildError> {
        Ok(TraceLog {
            span_id: self.span_id,
            timestamp: self.timestamp.ok_or_else(|| BuildError::missing_field("timestamp"))?,
            severity_number: self.severity_number.ok_or_else(|| BuildError::missing_field("severity_number"))?,
            severity_text: self.severity_text.ok_or_else(|| BuildError::missing_field("severity_text"))?,
            service_name: self.service_name.ok_or_else(|| BuildError::missing_field("service_name"))?,
            body: self.body.ok_or_else(|| BuildError::missing_field("body"))?,
            attributes: self.attributes,
        })
    }
}
