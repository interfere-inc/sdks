pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SpanDetail {
    #[serde(default)]
    pub attributes: String,
    #[serde(rename = "durationMs")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub duration_ms: f64,
    #[serde(rename = "endTimeUnixNano")]
    #[serde(default)]
    pub end_time_unix_nano: UInt64String,
    #[serde(rename = "httpMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_method: Option<String>,
    #[serde(rename = "httpUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_url: Option<String>,
    #[serde(default)]
    pub kind: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "parentSpanId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_span_id: Option<String>,
    #[serde(rename = "serviceName")]
    #[serde(default)]
    pub service_name: String,
    /// OTel resource service.version of the build that emitted the span, usually a commit; null for browser spans
    #[serde(rename = "serviceVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_version: Option<String>,
    /// OTel resource deployment.environment.name (or legacy deployment.environment) the service reported; null for browser spans
    #[serde(rename = "deploymentEnvironment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_environment: Option<String>,
    /// OTel resource host.name of the process that emitted the span; null when unreported
    #[serde(rename = "hostName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_name: Option<String>,
    #[serde(rename = "sessionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(rename = "spanId")]
    #[serde(default)]
    pub span_id: String,
    #[serde(rename = "startTimeUnixNano")]
    #[serde(default)]
    pub start_time_unix_nano: UInt64String,
    #[serde(rename = "statusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
    #[serde(rename = "statusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// UTC timestamp formatted as YYYY-MM-DD HH:mm:ss.
    #[serde(default)]
    pub timestamp: String,
}

impl SpanDetail {
    pub fn builder() -> SpanDetailBuilder {
        <SpanDetailBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SpanDetailBuilder {
    attributes: Option<String>,
    duration_ms: Option<f64>,
    end_time_unix_nano: Option<UInt64String>,
    http_method: Option<String>,
    http_url: Option<String>,
    kind: Option<String>,
    name: Option<String>,
    parent_span_id: Option<String>,
    service_name: Option<String>,
    service_version: Option<String>,
    deployment_environment: Option<String>,
    host_name: Option<String>,
    session_id: Option<String>,
    span_id: Option<String>,
    start_time_unix_nano: Option<UInt64String>,
    status_code: Option<String>,
    status_message: Option<String>,
    timestamp: Option<String>,
}

impl SpanDetailBuilder {
    pub fn attributes(mut self, value: impl Into<String>) -> Self {
        self.attributes = Some(value.into());
        self
    }

    pub fn duration_ms(mut self, value: f64) -> Self {
        self.duration_ms = Some(value);
        self
    }

    pub fn end_time_unix_nano(mut self, value: UInt64String) -> Self {
        self.end_time_unix_nano = Some(value);
        self
    }

    pub fn http_method(mut self, value: impl Into<String>) -> Self {
        self.http_method = Some(value.into());
        self
    }

    pub fn http_url(mut self, value: impl Into<String>) -> Self {
        self.http_url = Some(value.into());
        self
    }

    pub fn kind(mut self, value: impl Into<String>) -> Self {
        self.kind = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn parent_span_id(mut self, value: impl Into<String>) -> Self {
        self.parent_span_id = Some(value.into());
        self
    }

    pub fn service_name(mut self, value: impl Into<String>) -> Self {
        self.service_name = Some(value.into());
        self
    }

    pub fn service_version(mut self, value: impl Into<String>) -> Self {
        self.service_version = Some(value.into());
        self
    }

    pub fn deployment_environment(mut self, value: impl Into<String>) -> Self {
        self.deployment_environment = Some(value.into());
        self
    }

    pub fn host_name(mut self, value: impl Into<String>) -> Self {
        self.host_name = Some(value.into());
        self
    }

    pub fn session_id(mut self, value: impl Into<String>) -> Self {
        self.session_id = Some(value.into());
        self
    }

    pub fn span_id(mut self, value: impl Into<String>) -> Self {
        self.span_id = Some(value.into());
        self
    }

    pub fn start_time_unix_nano(mut self, value: UInt64String) -> Self {
        self.start_time_unix_nano = Some(value);
        self
    }

    pub fn status_code(mut self, value: impl Into<String>) -> Self {
        self.status_code = Some(value.into());
        self
    }

    pub fn status_message(mut self, value: impl Into<String>) -> Self {
        self.status_message = Some(value.into());
        self
    }

    pub fn timestamp(mut self, value: impl Into<String>) -> Self {
        self.timestamp = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SpanDetail`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attributes`](SpanDetailBuilder::attributes)
    /// - [`duration_ms`](SpanDetailBuilder::duration_ms)
    /// - [`end_time_unix_nano`](SpanDetailBuilder::end_time_unix_nano)
    /// - [`kind`](SpanDetailBuilder::kind)
    /// - [`name`](SpanDetailBuilder::name)
    /// - [`service_name`](SpanDetailBuilder::service_name)
    /// - [`span_id`](SpanDetailBuilder::span_id)
    /// - [`start_time_unix_nano`](SpanDetailBuilder::start_time_unix_nano)
    /// - [`timestamp`](SpanDetailBuilder::timestamp)
    pub fn build(self) -> Result<SpanDetail, BuildError> {
        Ok(SpanDetail {
            attributes: self.attributes.ok_or_else(|| BuildError::missing_field("attributes"))?,
            duration_ms: self.duration_ms.ok_or_else(|| BuildError::missing_field("duration_ms"))?,
            end_time_unix_nano: self.end_time_unix_nano.ok_or_else(|| BuildError::missing_field("end_time_unix_nano"))?,
            http_method: self.http_method,
            http_url: self.http_url,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            parent_span_id: self.parent_span_id,
            service_name: self.service_name.ok_or_else(|| BuildError::missing_field("service_name"))?,
            service_version: self.service_version,
            deployment_environment: self.deployment_environment,
            host_name: self.host_name,
            session_id: self.session_id,
            span_id: self.span_id.ok_or_else(|| BuildError::missing_field("span_id"))?,
            start_time_unix_nano: self.start_time_unix_nano.ok_or_else(|| BuildError::missing_field("start_time_unix_nano"))?,
            status_code: self.status_code,
            status_message: self.status_message,
            timestamp: self.timestamp.ok_or_else(|| BuildError::missing_field("timestamp"))?,
        })
    }
}
