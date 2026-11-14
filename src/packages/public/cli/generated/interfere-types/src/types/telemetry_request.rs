pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TelemetryRequest {
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub cleanup: f64,
    #[serde(rename = "createRelease")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub create_release: f64,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub discover: f64,
    #[serde(rename = "fileCount")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub file_count: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub preflight: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<ReleaseBuildResult>,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub total: f64,
    #[serde(rename = "totalBytes")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub total_bytes: f64,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub upload: f64,
}

impl TelemetryRequest {
    pub fn builder() -> TelemetryRequestBuilder {
        <TelemetryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TelemetryRequestBuilder {
    cleanup: Option<f64>,
    create_release: Option<f64>,
    discover: Option<f64>,
    file_count: Option<f64>,
    preflight: Option<f64>,
    result: Option<ReleaseBuildResult>,
    total: Option<f64>,
    total_bytes: Option<f64>,
    upload: Option<f64>,
}

impl TelemetryRequestBuilder {
    pub fn cleanup(mut self, value: f64) -> Self {
        self.cleanup = Some(value);
        self
    }

    pub fn create_release(mut self, value: f64) -> Self {
        self.create_release = Some(value);
        self
    }

    pub fn discover(mut self, value: f64) -> Self {
        self.discover = Some(value);
        self
    }

    pub fn file_count(mut self, value: f64) -> Self {
        self.file_count = Some(value);
        self
    }

    pub fn preflight(mut self, value: f64) -> Self {
        self.preflight = Some(value);
        self
    }

    pub fn result(mut self, value: ReleaseBuildResult) -> Self {
        self.result = Some(value);
        self
    }

    pub fn total(mut self, value: f64) -> Self {
        self.total = Some(value);
        self
    }

    pub fn total_bytes(mut self, value: f64) -> Self {
        self.total_bytes = Some(value);
        self
    }

    pub fn upload(mut self, value: f64) -> Self {
        self.upload = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TelemetryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`cleanup`](TelemetryRequestBuilder::cleanup)
    /// - [`create_release`](TelemetryRequestBuilder::create_release)
    /// - [`discover`](TelemetryRequestBuilder::discover)
    /// - [`file_count`](TelemetryRequestBuilder::file_count)
    /// - [`total`](TelemetryRequestBuilder::total)
    /// - [`total_bytes`](TelemetryRequestBuilder::total_bytes)
    /// - [`upload`](TelemetryRequestBuilder::upload)
    pub fn build(self) -> Result<TelemetryRequest, BuildError> {
        Ok(TelemetryRequest {
            cleanup: self.cleanup.ok_or_else(|| BuildError::missing_field("cleanup"))?,
            create_release: self.create_release.ok_or_else(|| BuildError::missing_field("create_release"))?,
            discover: self.discover.ok_or_else(|| BuildError::missing_field("discover"))?,
            file_count: self.file_count.ok_or_else(|| BuildError::missing_field("file_count"))?,
            preflight: self.preflight,
            result: self.result,
            total: self.total.ok_or_else(|| BuildError::missing_field("total"))?,
            total_bytes: self.total_bytes.ok_or_else(|| BuildError::missing_field("total_bytes"))?,
            upload: self.upload.ok_or_else(|| BuildError::missing_field("upload"))?,
        })
    }
}

