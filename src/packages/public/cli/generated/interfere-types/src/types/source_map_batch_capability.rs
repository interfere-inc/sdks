pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SourceMapBatchCapability {
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub version: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compression: Option<SourceMapBatchCapabilityCompression>,
    #[serde(rename = "maxBodyBytes")]
    #[serde(default)]
    pub max_body_bytes: i64,
    #[serde(rename = "maxFiles")]
    #[serde(default)]
    pub max_files: i64,
}

impl SourceMapBatchCapability {
    pub fn builder() -> SourceMapBatchCapabilityBuilder {
        <SourceMapBatchCapabilityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SourceMapBatchCapabilityBuilder {
    version: Option<f64>,
    compression: Option<SourceMapBatchCapabilityCompression>,
    max_body_bytes: Option<i64>,
    max_files: Option<i64>,
}

impl SourceMapBatchCapabilityBuilder {
    pub fn version(mut self, value: f64) -> Self {
        self.version = Some(value);
        self
    }

    pub fn compression(mut self, value: SourceMapBatchCapabilityCompression) -> Self {
        self.compression = Some(value);
        self
    }

    pub fn max_body_bytes(mut self, value: i64) -> Self {
        self.max_body_bytes = Some(value);
        self
    }

    pub fn max_files(mut self, value: i64) -> Self {
        self.max_files = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SourceMapBatchCapability`].
    /// This method will fail if any of the following fields are not set:
    /// - [`version`](SourceMapBatchCapabilityBuilder::version)
    /// - [`max_body_bytes`](SourceMapBatchCapabilityBuilder::max_body_bytes)
    /// - [`max_files`](SourceMapBatchCapabilityBuilder::max_files)
    pub fn build(self) -> Result<SourceMapBatchCapability, BuildError> {
        Ok(SourceMapBatchCapability {
            version: self.version.ok_or_else(|| BuildError::missing_field("version"))?,
            compression: self.compression,
            max_body_bytes: self.max_body_bytes.ok_or_else(|| BuildError::missing_field("max_body_bytes"))?,
            max_files: self.max_files.ok_or_else(|| BuildError::missing_field("max_files"))?,
        })
    }
}
